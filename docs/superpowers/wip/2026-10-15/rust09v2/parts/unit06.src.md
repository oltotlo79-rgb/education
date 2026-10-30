## UNIT 06: Config・AppState・DB の初期化

### この単元のゴール

- RustNote の起動の順番（`.env` → 設定 → アセットの束 → DB を開く → マイグレーション → `AppState` → ルーター → 待ち受け）と、その順番にした理由を説明できる
- 設定を環境変数から**起動のときに1回だけ**読み、`Config` 型にまとめ、秘密の値（Turso のトークン）を `Secret` 型で表示から隠す理由を言える
- 設定の間違い・DB の場所の間違い・アセットの束が無いときに、**どの段階で、どんな文で止まるか**を、実際の起動で確かめられる
- `.env` と環境変数の優先の順番、ポートが使われているときの表示の順番（RustNote に残る改善点）を説明できる

### たとえ話でつかむ

お店の開店の準備を考えます。

- 店長は、朝いちばんに**今日の指示書**（どの倉庫から仕入れるか、何時に開けるか）を読みます。仕事の途中で、何度も指示書を読み直したりはしません。読んだ内容は、手元のメモ（`Config`）にまとめます。
- 指示書には、**金庫の暗証番号**も書いてあります。メモに写すときは、暗証番号の所を「＊＊＊＊」にしておき、メモを人に見せても漏れないようにします。
- 準備には**順番**があります。看板（アセット）が届いていなければ、倉庫を開けに行く前にやめます。倉庫（DB）を開けたら、棚の配置（マイグレーション）を今日の形に直します。全部そろってから、従業員みんなが使う道具箱（`AppState`）を作り、最後に**入口を開けます**。
- 入口を開けてから「準備ができていなかった」と分かると、お客さんが困ります。だから、**失敗しそうなことは、入口を開ける前に**済ませます。

| 開店の準備 | RustNote の起動（`src/main.rs`） |
|---|---|
| 今日の指示書 | 環境変数（と、開発のときの `.env`） |
| 手元のメモ | `Config::from_env()`（起動のときに1回だけ作る） |
| 暗証番号を＊にする | `Secret` 型（`{:?}` で `Secret(********)` と表示） |
| 看板が届いているか | `AssetBundle::load()`（無ければ、DB に触る前に止める） |
| 倉庫を開けて、棚を直す | `Db::open` と `db::migrate` |
| 道具箱 | `AppState::new(db)`（ルーターに登録して、全ページで共有。第6巻 UNIT 07） |
| 入口を開ける | `topcoat::start(router)` |

> **用語:** 環境変数（かんきょうへんすう：プログラムの外から渡す、名前と値の組。`DATABASE_URL=rustnote.db` のように書く。プログラムを書き換えずに、動かす場所ごとに設定を変えられる）

<!-- -->

> **用語:** .env（ドットエンブ：環境変数を書いておくファイル。開発のときに、毎回コマンドで設定しなくて済むように使う。秘密が入るので Git には入れない）

### 図で理解する

起動の順番と、それぞれで止まるときの文です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="14" width="260" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="150" y="36" font-size="12" fill="#0f172a" text-anchor="middle">① .env を読む（無くてもよい）</text>
<rect x="20" y="62" width="260" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="150" y="84" font-size="12" fill="#0f172a" text-anchor="middle">② Config::from_env()</text>
<rect x="20" y="110" width="260" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="150" y="132" font-size="12" fill="#0f172a" text-anchor="middle">③ AssetBundle::load()</text>
<rect x="20" y="158" width="260" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="150" y="180" font-size="12" fill="#0f172a" text-anchor="middle">④ Db::open（DB を開く）</text>
<rect x="20" y="206" width="260" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="150" y="228" font-size="12" fill="#0f172a" text-anchor="middle">⑤ db::migrate（表を最新に）</text>
<rect x="20" y="254" width="260" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="150" y="276" font-size="12" fill="#0f172a" text-anchor="middle">⑥ AppState・ルーター・「起動しました」</text>
<rect x="20" y="294" width="260" height="30" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="150" y="314" font-size="12" fill="#0f172a" text-anchor="middle">⑦ topcoat::start（待ち受け）</text>
<line x1="150" y1="48" x2="150" y2="58" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0601)"/>
<line x1="150" y1="96" x2="150" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0601)"/>
<line x1="150" y1="144" x2="150" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0601)"/>
<line x1="150" y1="192" x2="150" y2="202" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0601)"/>
<line x1="150" y1="240" x2="150" y2="250" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0601)"/>
<text x="310" y="84" font-size="11" fill="#b91c1c">設定を読み込めませんでした（トークンが無い など）</text>
<text x="310" y="132" font-size="11" fill="#b91c1c">アセットの束（assets/）がありません → DB のファイルは作られない</text>
<text x="310" y="180" font-size="11" fill="#b91c1c">DB（…）を開けませんでした（フォルダが無い など）</text>
<text x="310" y="228" font-size="11" fill="#b91c1c">マイグレーションに失敗しました</text>
<text x="310" y="314" font-size="11" fill="#b91c1c">サーバーを起動できませんでした（ポートが使われている など）</text>
<line x1="280" y1="80" x2="304" y2="80" stroke="#dc2626" stroke-width="2"/>
<line x1="280" y1="128" x2="304" y2="128" stroke="#dc2626" stroke-width="2"/>
<line x1="280" y1="176" x2="304" y2="176" stroke="#dc2626" stroke-width="2"/>
<line x1="280" y1="224" x2="304" y2="224" stroke="#dc2626" stroke-width="2"/>
<line x1="280" y1="310" x2="304" y2="310" stroke="#dc2626" stroke-width="2"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 起動の順番。失敗しやすく、ほかに影響を残さないものから順に確かめ、全部そろってから待ち受けを始める。どの段階で止まっても、「何をしようとして失敗したか」が1行目に出る</div>
</div>

- 設定（②）とアセットの束（③）は、**何も書き換えない**確かめです。ここで止まれば、DB のファイルも作られません。DB（④⑤）は、開いた時点でファイルができ、マイグレーションで表が書き換わるので、そのあとにします。
- 「起動しました」（⑥）は、待ち受け（⑦）の**前に**表示しています。⑦で失敗すると、表示と本当の状態が食い違います。これは RustNote に残る改善点です（問2・第6巻 UNIT 02）。

環境変数は、2つの所で読まれます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0602" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">PowerShell の $env:…</text>
<text x="120" y="64" font-size="11" fill="#475569" text-anchor="middle">（Fly.io では secrets・env）</text>
<text x="120" y="86" font-size="11" fill="#047857" text-anchor="middle">こちらが先（勝つ）</text>
<rect x="20" y="120" width="200" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="144" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">.env のファイル</text>
<text x="120" y="164" font-size="11" fill="#475569" text-anchor="middle">（開発用。Git に入れない）</text>
<text x="120" y="186" font-size="11" fill="#475569" text-anchor="middle">まだ無い名前だけを足す</text>
<rect x="300" y="60" width="120" height="100" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="104" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">環境変数</text>
<text x="360" y="124" font-size="11" fill="#0f172a" text-anchor="middle">（プロセスの中）</text>
<rect x="490" y="30" width="210" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="595" y="54" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Config::from_env()</text>
<text x="595" y="76" font-size="11" fill="#0f172a" text-anchor="middle">DATABASE_URL</text>
<text x="595" y="92" font-size="11" fill="#0f172a" text-anchor="middle">DATABASE_AUTH_TOKEN</text>
<rect x="490" y="120" width="210" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="144" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">topcoat::start</text>
<text x="595" y="166" font-size="11" fill="#0f172a" text-anchor="middle">HOST・PORT</text>
<line x1="220" y1="60" x2="296" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0602)"/>
<line x1="220" y1="160" x2="296" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0602)"/>
<line x1="420" y1="96" x2="486" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0602)"/>
<line x1="420" y1="124" x2="486" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0602)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 環境変数の流れ。dotenvy は、.env の値のうち、まだ環境変数に無い名前だけを足す。DB の設定は Config が、待ち受ける場所は topcoat::start が読む</div>
</div>

### まず動かす最小コード

#### 例6-1: 新しい DB で起動し、もう一度起動する

コピーした `rustnote` フォルダで、アセットの束を作ってから（第6巻 UNIT 08）、新しい DB のファイル `first.db` を指定して起動します。

```powershell
topcoat asset bundle
$env:DATABASE_URL = "first.db"
cargo run
```

**▼ 実行結果**

```text
@@OUT:u06_first@@
```

一度止めて（Ctrl+C）、同じ DB で、もう一度起動します。

```text
@@OUT:u06_second@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 1回目は、表を作ってから待ち受ける

- 1回目は、マイグレーションが3つ（ユーザー・ノート・セッションの表）適用されてから、「RustNote を起動しました」が出ました。`/health` は `{"status":"ok"}` です。
- 2回目は、マイグレーションの行がありません。`db::migrate` は、`schema_migrations` の表に「どこまで適用したか」を記録しているので、もう適用したものは流しません（第13巻 UNIT 02）。起動するたびに `migrate` を呼んでも、**新しいものだけ**が流れます。

##### 解説2: 設定は、起動のときに1回だけ読む

- `DATABASE_URL` は、`Config::from_env()` が、起動のときに1回だけ読みます。読んだ値は `Config` にまとめ、DB を開くのに使ったら、あとは `AppState`（の中の `Db`）だけを、全ページに渡します。
- ページの中で、毎回 `std::env::var("DATABASE_URL")` を読む書き方にしないのは、**途中で値が変わったり、読めなかったりしても困らない**ようにするためです。起動のときに読めれば、そのあとは設定で失敗しません。

### もう一歩: 実務に近い形

#### 例6-2: 設定を表示しても、秘密は出ない

設定を確かめたいとき、ログに `Config` を丸ごと表示したくなります。トークンのような秘密が入っていても大丈夫かを確かめます。

> **▼ このコードがやること（先に日本語で）:** 環境変数から `Config` を作り、`{:?}` で表示します。リモートの DB か（`is_remote()`）と、待ち受ける場所（`listen_address()`）も表示します。

```rust
@@FILE:book/rustnote/examples/u06_config.rs@@
```

3通りの環境変数で動かします（2つ目のトークンは、説明のための作り物です）。

```text
@@OUT:u06_config@@
```

- 環境変数が無いときは、`DATABASE_URL` の既定の `rustnote.db`（ローカルのファイル）になり、待ち受けは `127.0.0.1:3000` です。
- Turso の URL とトークンを入れると、`database_auth_token: Some(Secret(********))` と表示され、`very-secret-token` は**どこにも出ません**。`Secret` 型の `Debug` を、中身の代わりに `********` を書くように、自分で書いているからです（`src/config.rs`）。
- `http://127.0.0.1:8080`（手元で動かす libSQL のサーバー）は、リモートですが、トークンが無くても作れました。`Config::new` は、インターネットの向こう（`libsql://`・`https://`・`wss://`）のときだけ、トークンを必須にしています（第1巻 UNIT 09（準備中）の `turso dev`）。

### よくあるエラーと読み方

#### エラー1: Turso の URL なのに、トークンが無い

```text
@@OUT:u06_err_token@@
```

- **何が起きたか:** 1行目の `Error: 設定を読み込めませんでした` が「何をしようとして失敗したか」、`Caused by:` の下が「なぜ」です。`main` が `.context("設定を読み込めませんでした")` で付けた文と、`ConfigError::MissingAuthToken` の文が、上から順に並んでいます（第4巻 UNIT 05 の anyhow）。
- **直し方:** `$env:DATABASE_AUTH_TOKEN` にトークンを入れます。本番（Fly.io）では、トークンは `fly secrets set` で渡し、ファイルにも `fly.toml` にも書きません（第12巻 UNIT 05（準備中））。
- **学べること:** 設定の間違いは、**DB にも、ネットワークにも触る前に**見つけています。トークンが無いまま Turso に接続しに行って、時間がかかってから分かりにくいエラーになるより、ずっと分かりやすくなります。

#### エラー2: DB のファイルを、無いフォルダの中に指定した

```text
@@OUT:u06_err_dbpath@@
```

- **何が起きたか:** 「DB（`no_such_dir\first.db`）を開けませんでした」で、その下が libSQL の文です。`Unable to open connection to local database …: 14` の 14 は、SQLite の「ファイルを開けない」（`SQLITE_CANTOPEN`）の番号です（第13巻 UNIT 02）。SQLite は、DB の**ファイル**は作ってくれますが、**フォルダ**は作りません。
- **直し方:** フォルダを先に作る（`New-Item -ItemType Directory no_such_dir`）か、場所を直します。Fly.io では、`/data` のフォルダを Dockerfile で作り、そこに DB を置きます（第12巻 UNIT 02（準備中））。

#### エラー3: アセットの束が無い

```text
@@OUT:u06_err_bundle@@
```

- **何が起きたか:** アセットの束が無いので、③で止まりました。そして、`nobundle.db` のファイルは**作られていません**（`False`）。束を確かめるのを、DB を開くより前にしているからです。
- **学べること:** 起動に失敗したときに、「途中まで作られた DB」のような**後片付けが要るもの**を残さない順番です。設定と束のように、何も書き換えない確かめを先にします。

### RustNote ではここで使う

起動の順番は、`src/main.rs` の `main` そのものです（抜き出しています）。

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env が無くてもエラーにしない（本番では環境変数を直接設定する）
    dotenvy::dotenv().ok();

    let config = Config::from_env().context("設定を読み込めませんでした")?;

    // `topcoat asset bundle`（または `topcoat dev`）が実行ファイルの隣に作る assets/ を読む。
    // 無ければ DB に触る前に止める
    let assets = AssetBundle::load().context(
        "アセットの束（assets/）がありません。先に `topcoat asset bundle` を実行するか、`topcoat dev` で起動してください",
    )?;

    let db = Db::open(&config)
        .await
        .with_context(|| format!("DB（{}）を開けませんでした", config.database_url))?;
    let applied = db::migrate(&db)
        .await
        .context("マイグレーションに失敗しました")?;
```

- 1行に1段階で、それぞれに `.context(…)` で「何をしようとしたか」を付けています。だから、エラー1〜3のように、止まった段階が1行目で分かります。
- `dotenvy::dotenv().ok()` の `.ok()` は、「`.env` が無いのは失敗ではない」という意味です。本番には `.env` を置かず、環境変数を直接設定します。

`src/config.rs` の `Secret` は、`Debug` を自分で書き、`Display` は書いていません（抜き出しています）。

```rust
// {:?} で表示されても中身を出さない
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}
```

- 中身が本当に要る所（DB に接続する所）だけで、`expose()` という目立つ名前のメソッドで取り出します。`expose()` を検索すれば、秘密を使っている所が全部見つかります（問3）。

### 練習問題

**問1:** `.env` に `PORT=3010` と書いて起動すると、何番で待ち受けるでしょうか。`.env` はそのままで、`$env:PORT = "3011"` としてから起動すると、どうなるでしょうか。

**問2:** RustNote を1つ起動したまま、2つ目の窓で、もう1つ起動すると、どう表示されるでしょうか。表示の順番の、どこが困るでしょうか。

**問3:** 秘密の値を、`println!("トークン: {token}")` と `{}` で表示しようとすると、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
@@OUT:u06_dotenv1@@
```

```text
@@OUT:u06_dotenv2@@
```

- `.env` だけのときは 3010 番、環境変数も設定したときは 3011 番でした。`dotenvy::dotenv()` は、`.env` の値のうち、**まだ環境変数に無い名前だけ**を足します。すでにある環境変数は、上書きしません。
- だから、開発用の値は `.env` に書いておき、その場だけ変えたいときは `$env:` で上書きできます。本番では、Fly.io が環境変数を設定するので、`.env` が紛れ込んでも、本番の値が勝ちます（そもそも `.dockerignore` で `.env` をイメージに入れません。第12巻 UNIT 02（準備中））。

**問2の解答:**

```text
@@OUT:u06_err_port@@
```

- 2つ目は、`RustNote を起動しました: http://127.0.0.1:3000` と表示してから、`サーバーを起動できませんでした` で止まりました。`os error 10048` は、Windows の「アドレスはもう使われています」です。
- 困るのは、**「起動しました」と出たのに、起動していない**ことです。ログだけを見る人は、勘違いします。第6巻 UNIT 02 の例2-3のように、`TcpListener::bind` で先に場所を押さえてから表示し、`topcoat::serve` に渡せば直せます（RustNote のコードは変えていません）。

**問3の解答:**

`errs06/e06_display.rs` を `examples` に置いてビルドしました。

```rust
@@FILE:book/errs06/e06_display.rs@@
```

```text
@@OUT:u06_q3@@
```

- 「`Secret` は `Display` を実装していません」（E0277）で、**コンパイルが止まりました**。`Secret` には、わざと `Display` を書いていないので、`{}` で表示しようとすると、うっかりでも気づけます。
- `note:` が勧める `{:?}` なら表示できますが、出るのは `Secret(********)` です。中身が本当に要るときは `token.expose()` と書き、**秘密を使っている所が、コードを読めば分かる**ようにします。

### まとめ

- RustNote の起動は、`.env` → `Config::from_env()` → `AssetBundle::load()` → `Db::open` → `db::migrate` → `AppState`・ルーター → `topcoat::start` の順。何も書き換えない確かめを先にし、全部そろってから待ち受ける。各段階に `.context(…)` を付けて、止まった段階を1行目で分かるようにする。
- 設定は起動のときに1回だけ読んで `Config` にまとめ、秘密は `Secret` で `{:?}` から隠し、`{}` ではコンパイルを止める。`.env` は、まだ無い名前だけを足すので、環境変数が勝つ。
- マイグレーションは毎回の起動で呼び、新しいものだけが流れる。「起動しました」を待ち受けの前に出すと、ポートが使われているときに食い違うので、場所を押さえてから表示するのがよい。
