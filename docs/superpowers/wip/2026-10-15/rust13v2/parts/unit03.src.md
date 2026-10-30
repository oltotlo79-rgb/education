## UNIT 03: 設定と起動の順番

### この単元のゴール

- `src/config.rs` が、環境変数から `Config` を作る決まり（既定値・Turso ならトークンが要る・空のトークンは「無い」と同じ）を、6つの組み合わせの実際の結果で説明できる
- `Secret` 型が、`{:?}` ではトークンを隠し、`{}` ではそもそも表示させない仕組みを説明できる
- `src/main.rs` の起動の順番（`.env` → 設定 → CSS の束 → DB を開く → マイグレーション → ルーター → 待ち受け）を言え、どの段で止まると何が残るかを、本物の実行ファイルで確かめられる
- `src/lib.rs` と `src/main.rs` を分けている理由と、`pub mod` を1つ消すと何が起きるかを説明できる

### たとえ話でつかむ

お店の、朝の開店の準備を考えます。

- まず、**今日の決まり**（営業時間・レジの金額）を、店長の書き置き（`.env`）や本部からの連絡（環境変数）で確かめます。本部からの連絡と書き置きが食い違うときは、**本部の連絡が勝ち**ます。
- 次に、**看板と飾り**（CSS の束）がそろっているかを見ます。無ければ、倉庫を開ける前にやめます。
- それから、**倉庫**（DB）を開けて、棚の配置が今日の決まりどおりか（マイグレーション）を確かめ、足りない棚を組み立てます。
- 最後に、**シャッターを開けて**（待ち受け）、お客さんを入れます。
- 金庫の暗証番号（トークン）は、メモ（ログ）に書くときは `********` と伏せます。そもそも、店先の黒板（`{}` の表示）には書けないようにしてあります。
- 準備は、**手早く確かめられる物から**順にします。看板が無いと分かっているのに、倉庫の棚を組み立ててしまうと、片付けが増えるからです。

| 開店の準備 | RustNote の起動（`src/main.rs`） |
|---|---|
| 店長の書き置き | `.env`（`dotenvy::dotenv().ok()`。無くてもよい） |
| 本部からの連絡 | 環境変数（`.env` より優先） |
| 今日の決まり | `Config::from_env()`（`src/config.rs`） |
| 看板と飾り | `AssetBundle::load()`（CSS の束） |
| 倉庫を開けて棚を組む | `Db::open` と `db::migrate`（UNIT 02） |
| シャッターを開ける | `topcoat::start(router)`（`HOST`・`PORT` で待ち受け） |
| 暗証番号を伏せる | `Secret` 型（`{:?}` で `Secret(********)`） |

> **用語:** 環境変数（かんきょうへんすう：プログラムの外から渡す、名前と値の組。`DATABASE_URL=rustnote.db` のように書く。Fly.io では `fly secrets set` で設定する（第12巻））

<!-- -->

> **用語:** `.env`（ドット・エンブ：開発のとき、環境変数をまとめて書いておくファイル。RustNote は起動のときに読むが、すでにある環境変数は上書きしない）

### 図で理解する

`src/main.rs` の起動の順番と、それぞれの段で止まったときに残る物です（例3-3の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="160" height="36" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="100" y="39" font-size="12" fill="#0f172a" text-anchor="middle">① .env を読む（無くてよい）</text>
<rect x="20" y="64" width="160" height="36" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="100" y="87" font-size="12" fill="#1d4ed8" text-anchor="middle">② Config::from_env</text>
<rect x="20" y="112" width="160" height="36" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="100" y="135" font-size="12" fill="#1d4ed8" text-anchor="middle">③ AssetBundle::load</text>
<rect x="20" y="160" width="160" height="36" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="183" font-size="12" fill="#047857" text-anchor="middle">④ Db::open</text>
<rect x="20" y="208" width="160" height="36" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="231" font-size="12" fill="#047857" text-anchor="middle">⑤ db::migrate</text>
<rect x="20" y="256" width="160" height="36" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="100" y="279" font-size="11" fill="#c2410c" text-anchor="middle">⑥ ルーター・「起動しました」</text>
<rect x="200" y="256" width="150" height="36" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="275" y="279" font-size="12" fill="#c2410c" text-anchor="middle">⑦ topcoat::start</text>
<line x1="100" y1="52" x2="100" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<line x1="100" y1="100" x2="100" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<line x1="100" y1="148" x2="100" y2="158" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<line x1="100" y1="196" x2="100" y2="206" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<line x1="100" y1="244" x2="100" y2="254" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<line x1="180" y1="274" x2="196" y2="274" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0301)"/>
<text x="380" y="88" font-size="11" fill="#b91c1c">Turso なのにトークンが無い → ここで止まる</text>
<text x="380" y="104" font-size="11" fill="#475569">DB のファイルは作られない</text>
<text x="380" y="136" font-size="11" fill="#b91c1c">CSS の束（assets/）が無い → ここで止まる</text>
<text x="380" y="152" font-size="11" fill="#475569">DB のファイルは作られない</text>
<text x="380" y="214" font-size="11" fill="#047857">ここで DB のファイルができ、3つの版が当たる</text>
<text x="380" y="270" font-size="11" fill="#b91c1c">PORT=abc → ⑦で止まる。DB はもう作られ、</text>
<text x="380" y="286" font-size="11" fill="#b91c1c">「起動しました」も、もう表示されている</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 起動の順番。手早く確かめられる設定と CSS の束を先に見て、DB に触る前に止める。待ち受けのアドレス（HOST・PORT）だけは最後の topcoat::start で読むので、DB を作ってから止まる</div>
</div>

### まず動かす最小コード

#### 例3-1: src/config.rs の全文

> **▼ このファイルがやること（先に日本語で）:** 環境変数の `DATABASE_URL` と `DATABASE_AUTH_TOKEN` を読んで、`Config` を作ります。`DATABASE_URL` が無ければ `rustnote.db`、Turso の URL（`libsql://`・`https://`・`wss://`）なのにトークンが無ければ、エラーにします。トークンは、読んだらすぐ `Secret` 型に包み、`{:?}` で表示しても中身が出ないようにします。待ち受けのアドレスを、起動の知らせのために組み立てる関数 `listen_address` もあります。

```rust
@@FILE:rustnote/src/config.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Secret は「隠す」と「出させない」の2つで守る

- `Debug` を `derive` せずに手で書き、`Secret(********)` とだけ表示します。だから、`Config` を `{:?}` でログに出しても（`Config` は `#[derive(Debug)]`）、トークンの中身は出ません。
- `Display` は**実装していません**。うっかり `{}` で表示しようとすると、コンパイルエラーになります（エラー2）。
- 中身が本当に要る所（`src/db.rs` の `Db::open` で Turso につなぐ所）だけで、`expose()` を呼びます。目立つ名前なので、`expose` で探せば、秘密を使っている場所がすぐ分かります。

##### 解説2: from_env と new を分ける

- `from_env` は「環境変数を読む」だけで、決まりの確かめは `new` に任せています。テストでは、環境変数を書き換えずに `Config::new(…)` で決まりを確かめられます（`#[cfg(test)]` の4つのテスト）。
- `is_remote`（ネットの向こうの DB か）と `requires_auth_token`（トークンが要るか）は、別の判定です。`http://` と `ws://` は「手元で動かす sqld（libSQL のサーバー）」のためのもので、リモートですが、トークンは要りません（例3-3の 6）。

##### 解説3: 環境変数が読めないときと、無いとき

- `read_var` は、無ければ `Ok(None)`、UTF-8 として読めなければ `ConfigError::NotUnicode` を返します。「無い」と「おかしい」を分けているので、既定値を使ってよい場合と、止めるべき場合を、取り違えません。
- 空の文字列のトークン（`DATABASE_AUTH_TOKEN=`）は、`filter(|t| !t.is_empty())` で「無い」と同じにしています。`.env.example` をコピーしたままの `DATABASE_AUTH_TOKEN=` の行が、「空のトークンで接続しようとする」事故にならないようにするためです。

#### 例3-2: src/main.rs と src/lib.rs の全文

> **▼ このファイルがやること（先に日本語で）:** `src/main.rs` は、起動の順番そのものです。`.env` を読み、設定を作り、CSS の束を読み、DB を開いてマイグレーションをかけ、ルーターを組み立てて待ち受けます。どこかで失敗したら、`anyhow` の `context` で「何をしようとして失敗したか」を付けて表示し、終わります。`src/lib.rs` は、アプリの中身を部品（モジュール）として公開するだけのファイルです。

```rust
@@FILE:rustnote/src/main.rs@@
```

```rust
@@FILE:rustnote/src/lib.rs@@
```

##### 解説4: 手早く確かめられる物から

- 設定（②）と CSS の束（③）は、ファイルや環境変数を読むだけなので、一瞬で確かめられます。DB を開く（④）とマイグレーション（⑤）は、ファイルを作ったり、ネットの向こうに接続したりします。だから、先に②③を確かめ、だめなら、DB に触る前に止めます（例3-3）。
- `main` が返す `anyhow::Result<()>` は、`Err` で終わると、`Error:` と、`context` で付けた文、`Caused by:` の下に元のエラーを表示して、終了の番号を 1 にします。

##### 解説5: main.rs は薄く、中身は lib.rs に

- アプリの中身は、全部 `src/lib.rs` から公開する部品（`rustnote::config` など）に置き、`src/main.rs` は、それを順に呼ぶだけです。こうすると、`tests/` のテストや `examples/` の練習のプログラムも、同じ部品を `rustnote::…` で使えます（第9巻 UNIT 04）。
- `pub mod` の1行が、そのファイル（`src/flash.rs` など）を、このクレートの部品にする宣言です。1行消すと、そのファイルは無いのと同じになります（エラー1）。

### もう一歩: 実務に近い形

#### 例3-3: 環境変数の組み合わせと、本物の起動

まず、`Config::from_env` が、6つの組み合わせをどう読むかを、練習用のプログラムで確かめます。

> **▼ このコードがやること（先に日本語で）:** `DATABASE_URL` と `DATABASE_AUTH_TOKEN` を、6通りに書き換えては、`Config::from_env()` を呼んで、結果（`Ok` の中身か、`Err` の文）を表示します。edition 2024 では、プログラムの中で環境変数を書き換える `set_var`・`remove_var` は `unsafe` なので、`unsafe { … }` の中で呼びます（エラー3）。

```rust
@@FILE:rustnote/examples/u03_config.rs@@
```

```powershell
cargo run -q --example u03_config
```

```text
@@OUTGREP:u03_config:^[ 1-6]@@
```

- 4（空のトークン）は、3（トークン無し）と同じエラーでした。5 は、トークンの中身が `Secret(********)` と隠れて表示されました。6 の `http://` は、リモート（`remote: true`）ですが、トークン無しで通りました。

次に、本物の実行ファイル（`cargo build` と `topcoat asset bundle` を済ませたもの）を、4つの設定で起動しました。それぞれのあとで、作業のフォルダの中のファイルも表示しました（`run_case` という小さなスクリプトで、標準エラー出力と、フォルダの中身を記録しました）。

**▼ ふつうに起動（`PORT=3014`・`DATABASE_URL=ok.db`）**

```text
@@OUT:u03_start_ok@@
@@OUT:u03_start_ok_health@@
```

**▼ Turso の URL で、トークンが無い**

```text
@@OUT:u03_start_token@@
```

**▼ CSS の束（assets/）が無いフォルダの実行ファイル**

```text
@@OUT:u03_start_assets@@
```

**▼ `PORT=abc`（数ではないポート）**

```text
@@OUT:u03_start_port@@
```

- ふつうの起動では、3つの版が当たり、`/health` が `200 OK` でした。`ok.db` ができました。
- トークンが無いときと、CSS の束が無いときは、②・③で止まり、**DB のファイルは作られません**でした（フォルダの中は、前の `ok.db` だけ）。
- `PORT=abc` は、⑦の `topcoat::start` で初めて読まれるので、その前に、**`port.db` を作って3つの版を当て**、しかも「**RustNote を起動しました: http://127.0.0.1:abc**」と表示してから、`PORT must be a valid port number`（PORT は正しいポートの番号でなければならない）で止まりました。起動の知らせが、待ち受けを始める**前**に出ているので、ログだけを見ると、起動したように見えてしまいます（改善点。問3）。

### よくあるエラーと読み方

#### エラー1: `src/lib.rs` から `pub mod flash;` を消した

```text
@@OUTHEAD:u03_err_mod:6@@
  …（同じエラーが続く）…
@@OUTGREP:u03_err_mod:^error: could not@@
```

- **何が起きたか:** `src/flash.rs` は、ファイルとしてはあるのに、`pub mod flash;` が無いので、クレートの部品になりません。`use crate::flash::…` と書いた 11 か所が、「クレートの根元に `flash` が見つかりません」（E0432）になりました。
- **読み方:** 同じエラーがたくさん出ても、原因は1つのことがよくあります。`could not find flash in the crate root`（根元に無い）なので、根元（`src/lib.rs`）の `mod` の宣言を見ます。
- **直し方:** `pub mod flash;` を戻します。新しいファイルを足したときも、`src/lib.rs`（または親のモジュール）に `mod` の1行を足すのを忘れないようにします。

#### エラー2: `Secret` を `{}` で表示しようとした（E0277）

```rust
@@FILE:errs_u03/e03_display.rs@@
```

```text
@@OUTGREP:u03_err_display:^(error\[|  -->|[0-9 ]+\||  = )@@
```

- **何が起きたか:** 「`Secret` は `Display` を実装していません」というエラーです。`Secret` は、わざと `Display` を実装していないので、`{}` では表示できません。
- **読み方:** `note:` の「`{:?}` を使えるかもしれません」のとおり、`{:?}` なら表示できますが、出るのは `Secret(********)` です。中身が要るなら、`expose()` を呼んだ所が「秘密を使う所」だと、読む人に分かるようにします。

#### エラー3: edition 2024 で、`set_var` を `unsafe` なしで呼んだ（E0133）

```rust
@@FILE:errs_u03/e03_setvar.rs@@
```

```text
@@OUTGREP:u03_err_setvar:^(error\[|  -->|[0-9 ]+\||  = )@@
```

- **何が起きたか:** 「`set_var` は `unsafe` な関数なので、`unsafe` のブロックが要ります」というエラーです。edition 2024 から、環境変数を書き換える `std::env::set_var` と `remove_var` は `unsafe` になりました。ほかのスレッドが同じ時に環境変数を読むと、OS によっては、こわれた値を読むおそれがあるからです。
- **直し方:** 例3-3のように、ほかのスレッドが環境変数を読まないと分かっている所で、`unsafe { … }` に入れ、`// SAFETY:` のコメントで、なぜ安全かを書きます。アプリの中で環境変数を書き換えるのは、なるべく避けます（RustNote は、読むだけです）。

### RustNote ではここで使う

- **本番の設定:** Fly.io では、`DATABASE_URL` と `DATABASE_AUTH_TOKEN` を `fly secrets set` で渡し、`.env` は使いません（第12巻）。`HOST=0.0.0.0`・`PORT=8080` は、`fly.toml` の `[env]` で渡します（UNIT 10）。
- **テスト:** `tests/common/mod.rs` は、`Config` を使わず、`Db::open_local` で一時フォルダの DB を直接開きます（UNIT 02）。設定の決まりは、`src/config.rs` の4つのユニットテストで確かめます。

```text
@@OUTGREP:u03_config_tests:^(running|test |test result)@@
```

- **改善点（この単元で見つかったこと）:** `PORT` が正しくないと、DB を作ってマイグレーションを当て、「起動しました」と表示してから止まります（例3-3）。問3で、直し方を考えます。

### 練習問題

**問1:** `.env` に `DATABASE_URL=…` と書き、同じ名前の環境変数も設定して起動すると、どちらが使われるでしょうか。

**問2:** `src/config.rs` のテスト `debug_output_hides_the_token` は、何を確かめているでしょうか。`Config` に、あとから秘密の項目（たとえばメールを送るサービスのパスワード）を足すときに、気をつけることは何でしょうか。

**問3:** 例3-3の `PORT=abc` では、DB のファイルを作ってから止まり、「起動しました」とも表示されました。どう直せばよいでしょうか。

### 解答と解説

**問1の解答:**

作業のフォルダの `.env` に `DATABASE_URL=libsql://from-dotenv.example` と書き、①環境変数にも `DATABASE_URL=libsql://from-env.example` を設定して起動、②環境変数を設定せずに起動、の2回を比べました（どちらもトークンが無いので、設定のエラーで止まり、どちらの値が使われたかが分かります）。

```text
@@OUTGREP:u03_q1_both:^(Error|Caused|    DATABASE)@@
```

```text
@@OUTGREP:u03_q1_dotenv:^(Error|Caused|    DATABASE)@@
```

- 環境変数があるときは、環境変数（`from-env`）が使われ、無いときだけ `.env`（`from-dotenv`）が使われました。`dotenvy::dotenv()` は、**すでにある環境変数を上書きしません**。
- だから、本番（Fly.io）で環境変数を設定していれば、間違って `.env` がまぎれ込んでいても、本番の値が勝ちます。逆に、手元で `.env` を書き換えたのに効かないときは、同じ名前の環境変数が残っていないかを確かめます。

**問2の解答:**

- `Config` 全体を `{:?}` で文字にしたとき、①トークンの中身（`very-secret-token`）が含まれないこと、②`database_auth_token: Some(Secret(********))` と表示されること、③`expose()` で取り出すと、ちゃんと中身が返ること、の3つを確かめています（例3-1のテストの結果のとおり、通っています）。
- 秘密の項目を足すときは、`String` のままではなく、`Secret` に包みます。`Config` は `#[derive(Debug)]` なので、`String` の項目は `{:?}` でそのまま表示されてしまいます。足したら、同じ形のテスト（中身が `{:?}` に出ない）も足します（第8巻 UNIT 10・第11巻 UNIT 07 の「記録に秘密を書かない」）。

**問3の解答:**

- **直し方1（先に確かめる）:** `PORT` と `HOST` も、②の設定のところで読んで確かめます（`Config` に `port: u16` を足し、数でなければ `ConfigError` にする）。そうすれば、DB に触る前に止まります。
- **直し方2（先に待ち受けの場所を取る）:** `topcoat::start` は、中で「`HOST`・`PORT` を読む → `TcpListener::bind` → `serve`」をしています（topcoat 0.11.0 の `serve.rs`）。`main` の中で、DB を開く前に `TcpListener::bind` までしておき、マイグレーションのあとで `topcoat::serve(listener, router)` を呼べば、ポートが使えないこと（ほかのプログラムが使っている、など）も、DB に触る前に分かります。
- **「起動しました」の知らせ:** どちらの直し方でも、知らせは、待ち受けの場所を取れた**あと**に出します。ログを読む人（や、Fly.io の見張り）が、「起動した」と思い込まないようにするためです。教材のコードは、まだ直していません（巻末の総合演習で扱います）。

### まとめ

- `src/config.rs` は、`DATABASE_URL`（無ければ `rustnote.db`）と `DATABASE_AUTH_TOKEN`（空は無いのと同じ）を読み、Turso の URL なのにトークンが無ければエラーにする。トークンは `Secret` に包み、`{:?}` では `Secret(********)`、`{}` ではコンパイルエラーにして守る。`.env` は、すでにある環境変数を上書きしない。
- `src/main.rs` は、.env → 設定 → CSS の束 → DB を開く → マイグレーション → ルーター → 待ち受けの順に動く。トークン無しと束無しは DB に触る前に止まり、`PORT=abc` は DB を作って「起動しました」と表示してから止まった（改善点）。
- `src/lib.rs` は部品を公開するだけで、`main.rs` は薄い。`pub mod` を1行消すと、そのファイルを使う 11 か所が E0432 になる。edition 2024 では `set_var` は `unsafe`。
