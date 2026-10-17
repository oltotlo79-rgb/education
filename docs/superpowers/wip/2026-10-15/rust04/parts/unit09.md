## UNIT 09: 設定型・環境変数・秘密

### この単元のゴール

- 環境変数を `std::env::var` で読み、「無い」「数にならない」などの失敗を扱える
- 設定を `Config` 型に集め、`Config::from_env()` で**起動時に1回だけ**読んで検証できる
- 既定値 → 設定ファイル → 環境変数、の順に値を重ねる仕組みを説明できる
- パスワードやトークンのような**秘密の値**を、`impl Debug` を手で書いて表示やログから隠せる

### たとえ話でつかむ

旅行に出る前の**持ち物チェック**を考えてみましょう。玄関を出る前にパスポート・財布・切符を確かめれば、忘れ物に気づいても取りに戻れます。空港に着いてから「パスポートが無い」と気づいたら手遅れです。

プログラムの設定も同じです。「DB の場所」「待ち受けるポート番号」「DB の合言葉（トークン）」を、**起動した直後に全部確かめて**、足りなければその場で止まります。動き始めてから、最初のリクエストが来た瞬間に「DB の場所が分からない」と気づくのでは、利用者に迷惑がかかります。

そして、パスポートの番号を大声で読み上げないのと同じように、**秘密の値は表示しない**ようにします。ログの画面は、思ったよりたくさんの人や仕組みが見るものだからです。

> **用語:** 環境変数（かんきょうへんすう：OS がプログラムに渡す「名前＝値」の組。同じプログラムでも、動かす場所（自分のパソコン・本番サーバー）ごとに違う値を渡せる。値はいつも文字列）

<!-- -->

> **用語:** 秘密（シークレット：パスワード・API キー・DB のトークンなど、知られると悪用される値。ソースコードや Git に入れてはいけない）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-18" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" font-size="13" fill="#475569" text-anchor="middle">右に行くほど優先（後から重ねたものが勝つ）</text>
<rect x="20" y="40" width="180" height="120" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="64" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">① 既定値（コード）</text>
<text x="110" y="92" font-size="12" fill="#0f172a" text-anchor="middle">HOST = 127.0.0.1</text>
<text x="110" y="114" font-size="12" fill="#0f172a" text-anchor="middle">PORT = 3000</text>
<rect x="270" y="40" width="180" height="120" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="64" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">② 設定ファイル</text>
<text x="360" y="92" font-size="12" fill="#0f172a" text-anchor="middle">DATABASE_URL =</text>
<text x="360" y="112" font-size="12" fill="#0f172a" text-anchor="middle">file:rustnote.db</text>
<text x="360" y="140" font-size="12" fill="#1e40af" text-anchor="middle">（開発用・Git に入れない）</text>
<rect x="520" y="40" width="180" height="120" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="610" y="64" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">③ 環境変数</text>
<text x="610" y="92" font-size="12" fill="#0f172a" text-anchor="middle">HOST = 0.0.0.0</text>
<text x="610" y="112" font-size="12" fill="#0f172a" text-anchor="middle">PORT = 8080</text>
<text x="610" y="140" font-size="12" fill="#c2410c" text-anchor="middle">（本番・秘密はここ）</text>
<line x1="200" y1="100" x2="266" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-18)"/>
<line x1="450" y1="100" x2="516" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-18)"/>
<line x1="610" y1="160" x2="610" y2="186" stroke="#475569" stroke-width="2"/>
<line x1="610" y1="186" x2="474" y2="200" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-18)"/>
<rect x="240" y="180" width="230" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="355" y="204" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Config（検証済み）</text>
<text x="355" y="224" font-size="12" fill="#047857" text-anchor="middle">0.0.0.0:8080 / file:rustnote.db</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-18: 設定の読み込み順。既定値 → ファイル → 環境変数の順に重ね、最後に検証して Config にする</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-19" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="150" height="70" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="95" y="60" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">起動</text>
<text x="95" y="82" font-size="12" fill="#c2410c" text-anchor="middle">main の最初</text>
<line x1="170" y1="65" x2="226" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-19)"/>
<rect x="230" y="30" width="200" height="70" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="330" y="58" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Config::from_env()</text>
<text x="330" y="80" font-size="12" fill="#1e40af" text-anchor="middle">ここで1回だけ全部検査</text>
<line x1="330" y1="100" x2="330" y2="136" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-04-19)"/>
<rect x="230" y="140" width="200" height="60" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="330" y="166" font-size="12" fill="#c2410c" text-anchor="middle">足りない・おかしい</text>
<text x="330" y="186" font-size="12" fill="#c2410c" text-anchor="middle">→ 理由を表示してすぐ終了</text>
<line x1="430" y1="65" x2="486" y2="65" stroke="#047857" stroke-width="2" marker-end="url(#ar-04-19)"/>
<rect x="490" y="30" width="210" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="595" y="58" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">AppState に入れて共有</text>
<text x="595" y="80" font-size="12" fill="#047857" text-anchor="middle">以後 env::var は呼ばない</text>
<rect x="490" y="140" width="210" height="60" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="595" y="166" font-size="12" fill="#475569" text-anchor="middle">ログ: {config:?}</text>
<text x="595" y="186" font-size="12" fill="#475569" text-anchor="middle">token: Secret(********)</text>
<line x1="595" y1="100" x2="595" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-19)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-19: 設定は起動時に1回だけ読んで検証し、秘密は表示で隠す</div>
</div>

### まず動かす最小コード

#### 例9-1: 環境変数 HOST と PORT を読む

> **▼ このコードがやること（先に日本語で）:** 環境変数 `HOST` と `PORT` を読み、どのアドレスで待ち受けるかを表示します。設定されていなければ `127.0.0.1` と `3000` を使い、`PORT` が数として読めなければ、理由を表示して終了します。

<!--CODE:design_lab/src/bin/u09_env.rs-->

何も設定せずに実行します。

```powershell
cargo run --bin u09_env
```

**▼ 実行結果**

<!--OUT:u09_env_default-->

次に、環境変数を設定して実行します。PowerShell では `$env:名前 = "値"` で、その PowerShell の画面の中だけで有効な環境変数を設定できます。

```powershell
$env:HOST = "0.0.0.0"
$env:PORT = "8080"
cargo run -q --bin u09_env
```

**▼ 実行結果**

<!--OUT:u09_env_set-->

`PORT` を数でない値や、大きすぎる値にしてみます。

```powershell
$env:PORT = "abc"
cargo run -q --bin u09_env
$env:PORT = "70000"
cargo run -q --bin u09_env
```

**▼ 実行結果**

<!--OUT:u09_env_bad-->

<!--OUT:u09_env_big-->

試し終わったら、設定した環境変数を消しておきます。

```powershell
Remove-Item Env:HOST, Env:PORT
```

> **ヒント:** WSL・macOS・Linux のシェル（bash など）では、`HOST=0.0.0.0 PORT=8080 cargo run -q --bin u09_env` のように、コマンドの前に `名前=値` を並べると、そのコマンドの間だけ環境変数を設定できます。

#### ▼ コードを1つずつ分解して解説

##### 解説1: `env::var` は `Result` を返す

```rust
let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
```

- `std::env::var("名前")` は、環境変数があれば `Ok(値)`、なければ `Err(...)` を返します。値はいつも `String` です。
- `unwrap_or_else(|_| 既定値)` は「`Err` なら既定値を使う」という意味です。`HOST` のように「無ければこれでよい」という既定値がある設定に使います。

##### 解説2: 失敗の理由を見分ける

```rust
Err(env::VarError::NotPresent) => 3000,
Err(env::VarError::NotUnicode(_)) => {
```

- `env::var` の失敗には2種類あります。`NotPresent`（設定されていない）と、`NotUnicode`（Rust の `String` として読めない文字が入っている）です。
- 「無い」は既定値で補ってよいですが、「おかしな値が入っている」は設定の間違いなので、**黙って既定値にせず**、止まって知らせるのが安全です。

##### 解説3: 文字列から数への変換で、範囲も確かめられる

- `text.parse::<u16>()` は、文字列を `u16`（0〜65535 の整数）に変換します。ポート番号はこの範囲なので、型を `u16` にしておけば、`"70000"` は `number too large to fit in target type`（型に収まらないほど大きい）として自動で断られます。UNIT 01 と同じく、**型の選び方そのものが検証になる**例です。
- `eprintln!` は、エラー用の出力（標準エラー出力）に表示するマクロです。`std::process::exit(1)` は、終了コード 1（失敗）でプログラムをすぐ終えます。

### もう一歩: 実務に近い形

例9-1 のように、あちこちで `env::var` を呼ぶと、次のような問題が起きます。

- どの環境変数が必要なのか、コード全体を読まないと分からない
- 設定の間違いに、その行が実行されるまで（リクエストが来るまで）気づかない
- テストで値を変えたいとき、プロセス全体の環境変数を書き換える必要がある（後で見るとおり、これは危険です）

そこで、設定を **`Config` 型**に集め、起動時に1回だけ読んで検証します。

#### 例9-2: Config::from_env() と、秘密を隠す Secret 型

> **▼ このコードがやること（先に日本語で）:** 待ち受けのアドレス・ポート、DB の場所、DB のトークン（秘密）を1つの `Config` にまとめて読み込みます。値は「既定値 → 設定ファイル → 環境変数」の順に重ね、足りない・おかしい値があれば理由つきで失敗します。トークンは、設定全体を表示しても `********` としか表示されないようにします。

この例では、環境変数と設定ファイルの代わりに、プログラムの中で作った表（`HashMap`）を使って4つの場面を再現し、最後に本物の環境変数からも読んでみます。

<!--CODE:design_lab/src/bin/u09_config.rs-->

```powershell
cargo run --bin u09_config
```

**▼ 実行結果**

<!--OUT:u09_config-->

> **注意:** 例の `rustnote-example.turso.io` や `eyJhbGciOi...very-secret` は説明用の値です。本物の DB の URL やトークンを、ソースコードや教材のメモに書いてはいけません。最後の行は、`DATABASE_URL` を設定していないパソコンで実行したときの結果です。

##### 解説4: Secret 型と、手で書いた `impl Debug`

```rust
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}
```

- `Config` には `#[derive(Debug)]` が付いているので、`{config:?}` で全部の項目を表示できます。起動時のログに設定を出しておくと、トラブルのときに役立ちます。
- しかし、トークンが `String` のままだと、**ログに秘密がそのまま出てしまいます**。そこで、UNIT 01 の newtype で `Secret` という型を作り、`Debug` を derive せずに**手で実装**して、中身の代わりに `********` を書くようにしました。実行結果の2行目で、`database_auth_token: Some(Secret(********))` となっていることを確認してください。
- `Secret` には `Display` を実装していないので、`{}` で表示しようとするとコンパイルエラーになります（練習問題2）。中身が本当に必要な場所（DB に接続するとき）では `expose()` を呼びます。**わざと目立つ名前**にしておくと、コードレビューで「ここで秘密を使っている」とすぐ分かります。

> **ヒント:** 同じ目的のクレートとして `secrecy` があり、秘密をメモリから消す工夫などもしてくれます。本書では仕組みを理解するために、自分で数行の型を書く方法を採りました。

##### 解説5: `from_env` と `from_lookup` を分ける

```rust
fn from_env() -> Result<Config, ConfigError> {
    Self::from_lookup(|key| std::env::var(key).ok())
}
```

- 本番では `Config::from_env()` を呼び、プロセスの本物の環境変数から読みます。
- 中身の処理は `from_lookup` に書き、「キーを渡すと値（があれば）を返す関数」を引数で受け取ります。本番ではこの関数が `env::var` を呼び、例やテストでは `HashMap` から値を引きます。UNIT 02 の「時計を差し替える」練習問題と同じ考え方で、**外の世界（環境変数）への依存を、引数で差し替えられる形**にしています。
- `impl Fn(&str) -> Option<String>` は「`&str` を受け取って `Option<String>` を返す、何かの関数（クロージャ）」という意味の引数です（UNIT 03 の `impl Trait`）。

##### 解説6: 既定値・必須・秘密・組み合わせの検査

```rust
// ④ 組み合わせの検査: リモートの DB にはトークンが必要
if database_url.starts_with("libsql://") && database_auth_token.is_none() {
    return Err(ConfigError::Missing("DATABASE_AUTH_TOKEN"));
}
```

- 設定には、①既定値があるもの（`HOST`・`PORT`）、②必須のもの（`DATABASE_URL`）、③あってもなくてもよい秘密（`DATABASE_AUTH_TOKEN`）があります。種類ごとに書き方を変えています。
- ④のように、**組み合わせ**で決まる決まりもあります。手元のファイルの DB（`file:...`）ならトークンは不要ですが、ネットワークの向こうの Turso の DB（`libsql://...`）には必要です。ケース3では、これを起動時に見つけて止まっています。
- `.filter(|t| !t.is_empty())` で、空文字の環境変数（`DATABASE_AUTH_TOKEN=` だけ書いてある状態）を「無い」と同じに扱っています。空のトークンで接続を試みて、分かりにくいエラーになるのを防ぐためです。

##### 解説7: 値を重ねる `layered`

```rust
move |key| env.get(key).or_else(|| file.get(key)).cloned()
```

- 「まず環境変数を見て、無ければ設定ファイルを見る」という関数を作っています。どちらにも無ければ `None` になり、`from_lookup` の中で既定値が使われます。これで図4-18 の「既定値 → ファイル → 環境変数」の順の重ね合わせができます。
- ケース2では、ファイルの `PORT = 3000` よりも環境変数の `PORT = 8080` が勝っています。開発用の値はファイルに、本番の値と秘密は環境変数に、という分け方がよく使われます。
- 設定ファイル（よく `.env` という名前にします）には秘密が入ることがあるので、**`.gitignore` に書いて Git に入れない**のが鉄則です。代わりに、値を空にした見本（`.env.example`）を Git に入れておくと、ほかの人が何を設定すればよいか分かります。

### よくあるエラーと読み方

#### エラー1: E0133 `set_var` は unsafe（edition 2024）

テストの準備などで、プログラムの中から環境変数を書き換えようとした場合です。

<!--CODE:errs/u09_err_e0133.rs-->

<!--OUT:u09_err_e0133-->

- **何が起きたか:** `call to unsafe function set_var is unsafe and requires unsafe block`（`set_var` は unsafe な関数なので、`unsafe` ブロックの中でしか呼べない）。edition 2024 から、`std::env::set_var` と `remove_var` は unsafe な関数になりました。
- **なぜ unsafe なのか:** 環境変数はプロセス全体で1つの共有の表です。あるスレッドが書き換えている最中に、別のスレッド（`cargo test` は既定でテストを並列に動かします）が読むと、プログラムが壊れる（未定義動作）ことがあるためです。
- **直し方:** `unsafe { ... }` で囲めば通りますが、「安全だと自分で保証する」責任を負うことになります。この単元の `from_lookup` のように、**環境変数を書き換えずに、値の取り出し方を差し替える設計**にすれば、unsafe はそもそも要りません。

#### エラー2: 実行時のパニック `NotPresent`

<!--CODE:design_lab/src/bin/u09_panic.rs-->

<!--OUT:u09_panic-->

- **何が起きたか:** `DATABASE_URL` が設定されていないので `env::var` が `Err(NotPresent)` を返し、`unwrap()` がパニック（プログラムの異常終了）を起こしました。終了コードは 101 です。`thread 'main' (55840)` の数字はスレッドの番号で、実行のたびに変わります。
- メッセージの `NotPresent` だけでは、**どの環境変数が無いのか**分かりません（行番号から探すしかありません）。
- **直し方:** 例9-2 のように、`ConfigError::Missing("DATABASE_URL")` のような**名前つきのエラー**にして、起動時に表示して終了します。anyhow を使うなら `env::var("DATABASE_URL").context("DATABASE_URL が設定されていません")?` のように文脈を付けます（UNIT 05）。

### RustNote ではここで使う

- RustNote では、DB の接続先とトークンなど、アプリ自身が使う設定を `Config` に集め、`main` の最初で `Config::from_env()` を1回だけ呼びます。失敗したら理由を表示して起動をやめます。読み込んだ `Config` は `AppState`（UNIT 04）に入れて共有し、ほかの場所では `env::var` を呼びません。
- 待ち受けの `HOST` / `PORT` は、RustNote で使う Web フレームワーク Topcoat の起動処理が環境変数から読みます（既定は `127.0.0.1:3000`。第6巻）。手元では何も設定しないので `http://127.0.0.1:3000` で動きます。Fly.io の本番では `HOST=0.0.0.0`・`PORT=8080` を設定し、トークンなどの秘密は `fly secrets` で設定します（第12巻）。
- RustNote でも、この単元と同じ `DATABASE_URL`・`DATABASE_AUTH_TOKEN` という名前を使います。ただし RustNote では、`DATABASE_URL` を設定しなければ手元のファイル `rustnote.db` を使う（既定値がある）ので、この単元の例のような「必須」の設定ではありません。項目の一覧は第9巻で確かめます。

### 練習問題

**問題1:** 環境変数 `APP_ENV` を読んで、`development`（開発）か `production`（本番）かを表す enum `AppEnv` にしてください。設定されていなければ `Development`、それ以外の値なら理由つきのエラーにします。`FromStr` を実装し、例9-2 と同じく「キーから値を引く関数」を受け取る形 `read_app_env(get: impl Fn(&str) -> Option<String>)` で書いてください。

**問題2:** 例9-2 の `Secret` を `println!("トークン: {token}")` のように `{}` で表示しようとすると、どうなるでしょうか。予想してから確かめ、それが「よいこと」である理由を説明してください。

**問題3:** 「設定は必要になったその場で `env::var` を呼べば、`Config` 型は要らない」という意見に対して、`Config` 型を起動時に1回だけ作る利点を3つ挙げてください。

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u09_ex1.rs-->

**▼ 実行結果**

<!--OUT:u09_ex1-->

- 文字列のまま `if app_env == "production"` と比べるコードがあちこちにあると、`"prod"` や `"Production"` のようなつづりの揺れで、こっそり開発用の動きになってしまいます。起動時に enum にしておけば、つづりの間違いはその場でエラーになり、その後は `AppEnv::Production` という**2つに1つの値**だけを扱えます。

**解答2:** コンパイルエラーになります（`Debug` だけを手で実装した `Secret` の場合）。

<!--CODE:errs/u09_err_display.rs-->

<!--OUT:u09_err_display-->

- `Secret` には `Display` を実装していないので、`{}` では表示できません（E0277）。「うっかり画面やログに秘密の中身を出す」書き方が、**コンパイルの段階で書けない**ようになっています。
- `{:?}` なら表示できますが、それは手で書いた `Secret(********)` です。中身が本当に必要なときは、`token.expose()` と明示的に書きます。

**解答3:**（例）

- **間違いに早く気づける:** 足りない設定・おかしな値を、起動した瞬間にまとめて見つけられます。最初のリクエストが来てから失敗するのを防げます。
- **必要な設定が一目で分かる:** `Config` の定義と `from_env` を読めば、アプリが何の設定を必要とするかが全部分かります。
- **テストしやすく、安全:** `from_lookup` のように値の取り出し方を差し替えられるので、プロセスの環境変数を書き換える（unsafe な）`set_var` を使わずにテストできます。秘密も `Secret` 型で一貫して隠せます。

### まとめ

- 環境変数は `env::var` で読み、いつも文字列です。「無い」は既定値で補い、「おかしい値」は止まって知らせます。
- 設定は `Config` 型に集め、`Config::from_env()` で起動時に1回だけ検証します。既定値 → ファイル → 環境変数の順に重ね、テストでは値の取り出し方を差し替えます。
- 秘密は `Secret` のような newtype に包み、`impl Debug` を手で書いて `********` と表示させます。`.env` は Git に入れません。
