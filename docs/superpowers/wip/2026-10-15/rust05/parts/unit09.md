## UNIT 09: tracing・構造化ログ・メトリクス

### この単元のゴール

- `println!` と tracing（トレーシング）のログの違い（**レベル**・**フィールド**・**スパン**）を説明し、`info!`・`warn!`・`error!`・`debug!` でフィールド付きのログを出せる
- ログの受け取り係（**subscriber**：サブスクライバー）をプログラムの最初に1回だけ登録し、`RUST_LOG` でログの細かさを変えられる
- `#[instrument]`（インストゥルメント）で関数の呼び出しを**スパン**（処理の区間）にし、かかった時間を記録できる。パスワードやトークンをログから外せる
- リクエストごとに `request_id` のスパンを作り、同時に来たリクエストのログを見分けられる。ログを JSON で出せる
- リクエストの数・エラーの数・応答時間の分布（**メトリクス**）を数えて、決まった形で書き出せる

### たとえ話でつかむ

病院の記録にたとえます。

- **`println!` は、メモ用紙の走り書き**です。「田中さん、熱あり」と書いても、いつ・どの患者の・どの検査の話なのか、後から探すのがたいへんです。
- **tracing のログは、カルテ**です。日時・重要度（ふつう・注意・緊急）・患者番号・検査の名前・値、と書く欄が決まっています。だから「患者番号 42 の、注意以上の記録だけ」のように、後から探したり数えたりできます。決まった欄に値を書く記録を、**構造化ログ**と呼びます。
- **スパンは、1回の通院の記録の束**です。受付から会計までを1つの束にし、その中に「問診」「検査」「診察」の小さな記録を入れます。それぞれに、始まりと終わりの時刻が書いてあるので、「この通院のどこに時間がかかったか」が分かります。
- **request_id は、受付番号**です。同じ時間にたくさんの患者さんがいても、受付番号で記録を分けられます。
- **メトリクスは、1日の集計表**です。来院者数・急患の数・待ち時間が10分未満の人の数、のように、1件ずつではなく**数**で病院の様子をつかみます。

> **用語:** ログ（log：プログラムが、何が起きたかを書き残す記録。サーバーでは、障害のときに原因をたどる、いちばん大事な手がかりになる）

<!-- -->

> **用語:** レベル（level：ログの重要度。重い順に ERROR（エラー）・WARN（注意）・INFO（ふつうの出来事）・DEBUG（調査用）・TRACE（もっと細かい調査用））

<!-- -->

> **用語:** フィールド（field：ログに付ける「名前 = 値」の欄。`user_id = 42` など。文章とは別に記録されるので、検索や集計に使える）

<!-- -->

> **用語:** スパン（span：処理の区間。始まりと終わりがあり、中で出したログはそのスパンの情報を引き継ぐ。スパンの中にスパンを入れて、木の形にできる）

### 図で理解する

まず、tracing でログが出るまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-091" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="200" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="110" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">あなたのコード</text>
<text x="110" y="70" font-size="11" fill="#0f172a" text-anchor="middle">info!(user_id = 42,</text>
<text x="110" y="88" font-size="11" fill="#0f172a" text-anchor="middle">"ノートを作成しました")</text>
<text x="110" y="114" font-size="10" fill="#475569" text-anchor="middle">何をどう出すかは書かない</text>
<rect x="250" y="20" width="200" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="350" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">イベント（1件のログ）</text>
<text x="350" y="66" font-size="10" fill="#0f172a" text-anchor="middle">レベル: INFO</text>
<text x="350" y="82" font-size="10" fill="#0f172a" text-anchor="middle">文章: ノートを作成しました</text>
<text x="350" y="98" font-size="10" fill="#0f172a" text-anchor="middle">フィールド: user_id = 42</text>
<text x="350" y="114" font-size="10" fill="#0f172a" text-anchor="middle">今いるスパン: request{…}</text>
<rect x="490" y="20" width="220" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">subscriber（受け取り係）</text>
<text x="600" y="68" font-size="10" fill="#0f172a" text-anchor="middle">① 出すか決める（RUST_LOG）</text>
<text x="600" y="88" font-size="10" fill="#0f172a" text-anchor="middle">② 形を決める（文字 / JSON）</text>
<text x="600" y="108" font-size="10" fill="#0f172a" text-anchor="middle">③ 書き出す（既定は標準出力）</text>
<line x1="210" y1="75" x2="246" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-091)"/>
<line x1="450" y1="75" x2="486" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-091)"/>
<rect x="250" y="160" width="220" height="40" rx="6" fill="#ffffff" stroke="#10b981"/>
<text x="360" y="184" font-size="10" fill="#0f172a" text-anchor="middle">INFO …: ノートを作成しました user_id=42</text>
<rect x="490" y="160" width="220" height="40" rx="6" fill="#ffffff" stroke="#10b981"/>
<text x="600" y="178" font-size="10" fill="#0f172a" text-anchor="middle">{"level":"INFO","fields":</text>
<text x="600" y="192" font-size="10" fill="#0f172a" text-anchor="middle">{"message":…,"user_id":42}}</text>
<path d="M560,130 V146 H360 V156" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-05-091)"/>
<line x1="600" y1="130" x2="600" y2="156" stroke="#10b981" stroke-width="2" marker-end="url(#ar-05-091)"/>
<text x="360" y="222" font-size="11" fill="#475569" text-anchor="middle">人が読む文字</text>
<text x="600" y="222" font-size="11" fill="#475569" text-anchor="middle">機械が読む JSON</text>
<text x="360" y="254" font-size="12" fill="#475569" text-anchor="middle">コードは「何が起きたか」だけを書き、どう出すかは subscriber が決める。subscriber を変えれば、コードを変えずに出し方を変えられる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: tracing のログの流れ。コードはイベント（レベル・文章・フィールド）を作るだけで、出すかどうかと形は subscriber が決める</div>
</div>

次に、スパンの木です。この後の例9-2で、1つのリクエストの処理の中で3つの関数を呼んだときのスパンを、実際の出力から図にしました。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-092" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="180" y="14" width="360" height="60" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="38" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">handle_request{path="/notes" user_id=42 method="GET"}</text>
<text x="360" y="60" font-size="10" fill="#475569" text-anchor="middle">全体: 約 0.14 秒（busy 764µs / idle 138ms）</text>
<rect x="10" y="120" width="220" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="144" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">check_password</text>
<text x="120" y="162" font-size="10" fill="#0f172a" text-anchor="middle">{user_id=42 password_len=11}</text>
<text x="120" y="176" font-size="10" fill="#475569" text-anchor="middle">idle 22.4ms</text>
<rect x="250" y="120" width="220" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="144" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">find_user</text>
<text x="360" y="162" font-size="10" fill="#0f172a" text-anchor="middle">{user_id=42}</text>
<text x="360" y="176" font-size="10" fill="#475569" text-anchor="middle">idle 31.0ms</text>
<rect x="490" y="120" width="220" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="144" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">search_notes</text>
<text x="600" y="162" font-size="10" fill="#0f172a" text-anchor="middle">{user_id=42 query="買い物"}</text>
<text x="600" y="176" font-size="10" fill="#475569" text-anchor="middle">idle 84.6ms</text>
<line x1="300" y1="74" x2="140" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-092)"/>
<line x1="360" y1="74" x2="360" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-092)"/>
<line x1="420" y1="74" x2="580" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-092)"/>
<text x="360" y="212" font-size="12" fill="#475569" text-anchor="middle">子のスパンの中で出したログには、親のスパンの情報（path・user_id など）も付く</text>
<text x="360" y="234" font-size="12" fill="#475569" text-anchor="middle">busy は CPU を使っていた時間、idle は .await で待っていた時間。どこで時間がかかったかが分かる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 1つのリクエストのスパンの木。#[instrument] の関数を呼ぶたびに子のスパンができ、区間ごとの時間が記録される</div>
</div>

### まず動かす最小コード

この単元では、tracing と tracing-subscriber を使います。`async_lab` フォルダで次を実行します。

```powershell
cargo add tracing@=0.1.44
cargo add tracing-subscriber@=0.3.23 --features env-filter,json
```

```text
    Updating crates.io index
      Adding tracing =0.1.44 to dependencies
             Features:
             + attributes
             + std
             + tracing-attributes
             - async-await
             - log
             - log-always
             - max_level_debug
             - max_level_error
             - max_level_info
             - max_level_off
             - max_level_trace
             - max_level_warn
             - release_max_level_debug
             - release_max_level_error
             - release_max_level_info
             - release_max_level_off
             - release_max_level_trace
             - release_max_level_warn
             - valuable
     Locking 1 package to highest Rust 1.98 compatible version
      Adding tracing-attributes v0.1.31
```

```text
    Updating crates.io index
      Adding tracing-subscriber =0.3.23 to dependencies
             Features:
             + alloc
             + ansi
             + env-filter
             + fmt
             + json
             + matchers
             + nu-ansi-term
             + once_cell
             + registry
             + serde
             + serde_json
             + sharded-slab
             + smallvec
             + std
             + thread_local
             + tracing
             + tracing-log
             + tracing-serde
             - chrono
             - local-time
             - parking_lot
             - regex
             - time
             - valuable
             - valuable-serde
             - valuable_crate
    Updating crates.io index
     Locking 12 packages to highest Rust 1.98 compatible versions
      Adding aho-corasick v1.1.5
      Adding lazy_static v1.5.1
      Adding matchers v0.2.0
      Adding nu-ansi-term v0.50.3
      Adding regex-automata v0.4.18
      Adding regex-syntax v0.8.11
      Adding sharded-slab v0.1.7
      Adding thread_local v1.1.10
      Adding tracing-log v0.2.0
      Adding tracing-serde v0.2.0
      Adding tracing-subscriber v0.3.23
      Adding valuable v0.1.1
```

- **tracing** は、ログを**出す側**（`info!` や `#[instrument]`）の道具です。tokio の作者たちが作っている、Rust のサーバーでよく使われるログのクレートです。
- **tracing-subscriber** は、ログを**受け取って書き出す側**（subscriber）の道具です。`env-filter` の機能で `RUST_LOG` による絞り込みが、`json` の機能で JSON の出力が使えるようになります。

> **ヒント:** この単元の実行結果は、色の記号を除くために、環境変数 `NO_COLOR` を設定して取りました。ふつうに実行すると、ターミナルでは INFO が緑、WARN が黄色のように色が付きます（tracing-subscriber 0.3.23 は、`NO_COLOR` が設定されていると色を付けません）。

#### 例9-1: レベルとフィールド付きのログを出す

> **▼ このコードがやること（先に日本語で）:** ログの受け取り係（subscriber）を登録してから、INFO・WARN・ERROR・DEBUG のログを出します。文章だけのログと、`user_id = 42` のような名前つきの値（フィールド）を添えたログを比べます。

```rust
use tracing::{debug, error, info, warn};

fn main() {
    // ログの受け取り係（subscriber）を1つ決めて、プログラム全体に登録する
    tracing_subscriber::fmt().init();

    // 文章だけのログ
    info!("RustNote を起動しました");
    // 名前つきの値（フィールド）を添えたログ。あとで検索・集計しやすい
    info!(user_id = 42, note_id = 7, "ノートを作成しました");
    // ? を付けると Debug 表示、% を付けると Display 表示でフィールドに入れる
    let path = "/notes/7";
    let elapsed = std::time::Duration::from_millis(35);
    warn!(%path, ?elapsed, "応答が遅めです");
    error!(
        error = "DB に接続できません",
        "ヘルスチェックに失敗しました"
    );
    // 既定では INFO 以上だけが表示されるので、この行は出ない
    debug!("細かい調査用のログ");
}
```

`src/bin/u09_fmt.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_fmt
```

**▼ 実行結果**

```text
2026-10-24T04:44:26.394409Z  INFO u09_fmt: RustNote を起動しました
2026-10-24T04:44:26.394547Z  INFO u09_fmt: ノートを作成しました user_id=42 note_id=7
2026-10-24T04:44:26.394558Z  WARN u09_fmt: 応答が遅めです path=/notes/7 elapsed=35ms
2026-10-24T04:44:26.394578Z ERROR u09_fmt: ヘルスチェックに失敗しました error="DB に接続できません"
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 1行のログに、時刻・レベル・出どころが付く

- 各行は「時刻（世界標準時、末尾の `Z`）・レベル・出どころ（`u09_fmt`）: 文章・フィールド」の順に並んでいます。時刻やレベルは、`println!` なら自分で書かなければならなかった部分です。
- 出どころ（target：ターゲット）は、ログを出したモジュールの名前です。ここではプログラム名の `u09_fmt` です。`RUST_LOG` でクレートごとに細かさを変えるときに使います（例9-3）。

##### 解説2: フィールドは「名前 = 値」で、文章とは別に記録される

- `info!(user_id = 42, note_id = 7, "ノートを作成しました")` の `user_id = 42` がフィールドです。文字の形では行の最後に `user_id=42 note_id=7` と付き、JSON の形では別の欄になります（例9-3）。
- 文章の中に `format!` で値を埋め込む（`"ユーザー42がノート7を作成"`）と、後から「user_id が 42 のログだけ」を探すのがたいへんです。値はフィールドに分けて書く、が構造化ログの基本です。
- `%path` は「`path` という名前のフィールドに、`Display`（`{}`）の形で入れる」、`?elapsed` は「`Debug`（`{:?}`）の形で入れる」という書き方です。変数の名前がそのままフィールドの名前になります。

##### 解説3: subscriber は、プログラムの最初に1回だけ登録する

- `tracing_subscriber::fmt().init()` は、「人が読む文字の形で、標準出力に書く」受け取り係を作り、プログラム全体に登録します。登録は1回だけです（2回するとどうなるかは「よくあるエラー2」）。
- 既定では INFO 以上だけが表示されるので、最後の `debug!` は出ていません。DEBUG も出す方法は、例9-3と練習問題1で学びます。

#### 例9-2: #[instrument] で、関数の呼び出しをスパンにする

> **▼ このコードがやること（先に日本語で）:** 1つのリクエストの処理 `handle_request` の中で、パスワードの確認・ユーザーの検索・ノートの検索の3つの関数を呼びます。どの関数にも `#[instrument]` を付けてスパンにし、スパンが閉じたときにかかった時間を表示する設定で実行します。パスワードはログに出さず、長さだけを記録します。

```rust
use std::time::Duration;

use tracing::{info, instrument, warn};
use tracing_subscriber::fmt::format::FmtSpan;

// #[instrument]: この関数を呼ぶたびに「スパン（処理の区間）」を作る。引数がフィールドとして記録される
#[instrument]
async fn find_user(user_id: i64) -> String {
    tokio::time::sleep(Duration::from_millis(20)).await;
    info!("ユーザーを見つけました");
    String::from("hanako")
}

#[instrument]
async fn search_notes(user_id: i64, query: &str) -> usize {
    tokio::time::sleep(Duration::from_millis(50)).await;
    let hits = 3;
    info!(hits, "検索しました");
    hits
}

// skip(password): パスワードはログに記録しない。fields で記録したい値を自分で足す
#[instrument(skip(password), fields(password_len = password.len()))]
async fn check_password(user_id: i64, password: &str) -> bool {
    tokio::time::sleep(Duration::from_millis(10)).await;
    warn!("この例では、どんなパスワードでも通します");
    true
}

// 1つのリクエストの処理。中で呼ぶ関数のスパンは、このスパンの「子」になる
#[instrument(fields(method = "GET"))]
async fn handle_request(path: &str, user_id: i64) {
    info!("リクエスト開始");
    check_password(user_id, "secret-pass").await;
    let name = find_user(user_id).await;
    let hits = search_notes(user_id, "買い物").await;
    info!(%name, hits, "リクエスト完了");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // スパンが閉じたとき（区間が終わったとき）にも1行出し、かかった時間を表示する
        .with_span_events(FmtSpan::CLOSE)
        // 時刻の代わりに「起動してからの秒数」を表示する（見やすくするため）
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    handle_request("/notes", 42).await;
}
```

`src/bin/u09_spans.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_spans
```

**▼ 実行結果**

```text
   0.001094500s  INFO handle_request{path="/notes" user_id=42 method="GET"}: u09_spans: リクエスト開始
   0.023701300s  WARN handle_request{path="/notes" user_id=42 method="GET"}:check_password{user_id=42 password_len=11}: u09_spans: この例では、どんなパスワードでも通します
   0.023786400s  INFO handle_request{path="/notes" user_id=42 method="GET"}:check_password{user_id=42 password_len=11}: u09_spans: close time.busy=139µs time.idle=22.4ms
   0.054912000s  INFO handle_request{path="/notes" user_id=42 method="GET"}:find_user{user_id=42}: u09_spans: ユーザーを見つけました
   0.054978900s  INFO handle_request{path="/notes" user_id=42 method="GET"}:find_user{user_id=42}: u09_spans: close time.busy=99.8µs time.idle=31.0ms
   0.139673400s  INFO handle_request{path="/notes" user_id=42 method="GET"}:search_notes{user_id=42 query="買い物"}: u09_spans: 検索しました hits=3
   0.139761200s  INFO handle_request{path="/notes" user_id=42 method="GET"}:search_notes{user_id=42 query="買い物"}: u09_spans: close time.busy=127µs time.idle=84.6ms
   0.139794000s  INFO handle_request{path="/notes" user_id=42 method="GET"}: u09_spans: リクエスト完了 name=hanako hits=3
   0.139812100s  INFO handle_request{path="/notes" user_id=42 method="GET"}: u09_spans: close time.busy=764µs time.idle=138ms
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[instrument] は、引数をフィールドにしたスパンを作る

- `#[instrument]` を付けた関数を呼ぶと、関数の名前のスパン（`find_user`）ができ、引数（`user_id=42`）がフィールドとして記録されます。関数の中で出したログには、行の頭に `find_user{user_id=42}` が付きます。
- `handle_request` の中で呼んだ3つの関数のスパンは、`handle_request` のスパンの**子**になります。だから、どの行にも `handle_request{path="/notes" user_id=42 method="GET"}:` が付いています。どのリクエストの、どの処理の中で出たログなのかが、1行を見るだけで分かります。
- `fields(method = "GET")` で、引数にない値もフィールドとして足せます。

##### 解説2: close の行で、区間にかかった時間が分かる

- `.with_span_events(FmtSpan::CLOSE)` を付けたので、スパンが閉じた（関数が終わった）ときに `close time.busy=… time.idle=…` の行が出ます。
- `time.busy` は、そのスパンの中で CPU を使って動いていた時間、`time.idle` は `.await` で待っていた時間です。`search_notes` は busy が 127µs（マイクロ秒）しかなく、idle が 84.6ms でした。ほとんどの時間を、DB の代わりの `sleep` で待っていたことが分かります。
- 遅いリクエストがあったとき、どのスパンの idle が長いか（何を待っていたか）を見れば、原因の見当がつきます。busy が長ければ、UNIT 07 の重い計算を疑います。

##### 解説3: パスワードは skip で外す

- `#[instrument]` は、既定では引数を**全部**フィールドにします。そのままだと、`check_password` の `password` もログに書き出されてしまいます。
- `skip(password)` で、`password` をフィールドから外しています。代わりに `fields(password_len = password.len())` で、長さだけを記録しています。実行結果に `password_len=11` はありますが、パスワードそのものは出ていません。
- パスワード・トークン・クッキーのような秘密の値は、ログに1度でも書かれると、ログを見られる人全員に漏れたことになります。`#[instrument]` を付けたら、引数の中に秘密がないかを必ず確かめましょう（「よくあるエラー3」）。

> **注意:** `uptime()` は「起動してからの秒数」を表示する設定で、この例を読みやすくするためだけに使っています。本番のサーバーでは、ほかのシステムのログと突き合わせられるように、ふつうの時刻（例9-1の形）を出します。

### もう一歩: 実務に近い形

#### 例9-3: JSON で出し、RUST_LOG で細かさを変える

> **▼ このコードがやること（先に日本語で）:** ログを1行1つの JSON で出す受け取り係を登録し、`RUST_LOG` 環境変数でログの細かさを決められるようにします。`#[instrument]` を付けた削除の関数の中で DEBUG と INFO のログを出し、`RUST_LOG` を付けない場合と、`debug` にした場合の出力を比べます。

```rust
use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;

#[instrument]
async fn delete_note(user_id: i64, note_id: i64) {
    debug!("DB から削除します");
    info!("ノートを削除しました");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // 1行1つの JSON で出す（ログを集めるサービスが読みやすい）
        .json()
        // RUST_LOG 環境変数で「どのくらい細かく出すか」を決める。無ければ info
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    delete_note(42, 7).await;
}
```

`src/bin/u09_json.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_json
```

**▼ 実行結果**

```text
{"timestamp":"2026-10-24T04:44:27.674141Z","level":"INFO","fields":{"message":"ノートを削除しました"},"target":"u09_json","span":{"note_id":7,"user_id":42,"name":"delete_note"},"spans":[{"note_id":7,"user_id":42,"name":"delete_note"}]}
```

次に、PowerShell で `RUST_LOG` を `debug` にして実行します。

```powershell
# この PowerShell の画面の中だけで、環境変数 RUST_LOG を設定する
$env:RUST_LOG = "debug"
cargo run -q --bin u09_json
# 終わったら消しておく
Remove-Item Env:RUST_LOG
```

```text
{"timestamp":"2026-10-24T04:44:28.850049Z","level":"DEBUG","fields":{"message":"DB から削除します"},"target":"u09_json","span":{"note_id":7,"user_id":42,"name":"delete_note"},"spans":[{"note_id":7,"user_id":42,"name":"delete_note"}]}
{"timestamp":"2026-10-24T04:44:28.850152Z","level":"INFO","fields":{"message":"ノートを削除しました"},"target":"u09_json","span":{"note_id":7,"user_id":42,"name":"delete_note"},"spans":[{"note_id":7,"user_id":42,"name":"delete_note"}]}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: JSON のログは、欄ごとに機械で読める

- `.json()` を付けると、1件のログが1行の JSON になります。`"level"`（レベル）・`"fields"`（文章とフィールド）・`"span"`（今いるスパン）・`"spans"`（親から順のスパンの並び）が、それぞれ別の欄です。
- ログを集めて検索するサービスの多くは、JSON のログなら「`level` が ERROR で、`span.user_id` が 42 のもの」のように、欄を指定して探せます。人が目で読むなら例9-1の文字の形、機械に集めさせるなら JSON、と使い分けます。

##### 解説2: RUST_LOG で、コードを変えずに細かさを変える

- `EnvFilter::try_from_default_env()` は、環境変数 `RUST_LOG` を読んで「どのレベルまで出すか」を決めます。`RUST_LOG` が無ければ `Err` になるので、`unwrap_or_else` で「`info` 以上」を既定にしています。
- `RUST_LOG=debug` にすると、DEBUG のログも出るようになりました。障害を調べるときだけ細かくする、ということが、プログラムを作り直さずにできます。
- `RUST_LOG` には、`u09_json=debug,reqwest=warn` のように、出どころごとに細かさを書けます（tracing-subscriber のドキュメントより）。自分のアプリは細かく、使っているクレートの細かいログは抑える、という使い方がよくされます。

##### 解説3: PowerShell の環境変数の書き方

- PowerShell では、`$env:名前 = "値"` で環境変数を設定します。設定はその PowerShell の画面（セッション）の中だけで有効です。`Remove-Item Env:名前` で消せます。
- WSL や macOS・Linux の bash では、`RUST_LOG=debug cargo run -q --bin u09_json` のように、コマンドの前に書くと、そのコマンドの間だけ設定されます。

#### 例9-4: リクエストごとに request_id を付けて、ログを見分ける

> **▼ このコードがやること（先に日本語で）:** 2つのリクエスト（`/notes` と、遅い `/slow`）が同時に来た場面を作ります。リクエストごとに `request_id`・`method`・`path` を持つスパンを作り、その中で処理を進めます。処理が終わったら、ステータスとかかった時間を1行のアクセスログにまとめます。200 ミリ秒以上かかったら WARN にします。ログは JSON で出します。

```rust
use std::time::{Duration, Instant};

use tracing::{Instrument, info, info_span, instrument, warn};

// DB への問い合わせの代わり。スパンの中で呼ばれるので、ログにはリクエストの情報も付く
#[instrument]
async fn load_notes(user_id: i64) -> usize {
    tokio::time::sleep(Duration::from_millis(30)).await;
    info!("ノートを読みました");
    3
}

// 1つのリクエストの処理
async fn handle(path: &str, user_id: i64) -> u16 {
    let count = load_notes(user_id).await;
    if path == "/slow" {
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    if count > 0 { 200 } else { 404 }
}

// アクセスログ: リクエストごとにスパンを作り、終わったら1行にまとめて記録する
async fn serve(request_id: u64, method: &str, path: &str, user_id: i64) {
    let span = info_span!("request", request_id, method, path);
    async {
        let started = Instant::now();
        let status = handle(path, user_id).await;
        let elapsed_ms = started.elapsed().as_millis() as u64;
        if elapsed_ms >= 200 {
            warn!(status, elapsed_ms, "遅いリクエスト");
        } else {
            info!(status, elapsed_ms, "リクエスト完了");
        }
    }
    .instrument(span)
    .await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        // 時刻を出さない（本に載せる出力を短くするため。本番では出す）
        .without_time()
        .init();

    // 2つのリクエストが同時に来た
    tokio::join!(serve(1, "GET", "/notes", 42), serve(2, "GET", "/slow", 7));
}
```

`src/bin/u09_access_log.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_access_log
```

**▼ 実行結果**

```text
{"level":"INFO","fields":{"message":"ノートを読みました"},"target":"u09_access_log","span":{"user_id":7,"name":"load_notes"},"spans":[{"method":"GET","path":"/slow","request_id":2,"name":"request"},{"user_id":7,"name":"load_notes"}]}
{"level":"INFO","fields":{"message":"ノートを読みました"},"target":"u09_access_log","span":{"user_id":42,"name":"load_notes"},"spans":[{"method":"GET","path":"/notes","request_id":1,"name":"request"},{"user_id":42,"name":"load_notes"}]}
{"level":"INFO","fields":{"message":"リクエスト完了","status":200,"elapsed_ms":37},"target":"u09_access_log","span":{"method":"GET","path":"/notes","request_id":1,"name":"request"},"spans":[{"method":"GET","path":"/notes","request_id":1,"name":"request"}]}
{"level":"WARN","fields":{"message":"遅いリクエスト","status":200,"elapsed_ms":292},"target":"u09_access_log","span":{"method":"GET","path":"/slow","request_id":2,"name":"request"},"spans":[{"method":"GET","path":"/slow","request_id":2,"name":"request"}]}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 同時に来たリクエストのログが混ざっても、request_id で分けられる

- 2つのリクエストは同時に進んだので、ログの行も混ざって出ています。1行目は request_id 2 の、2行目は request_id 1 の「ノートを読みました」です。
- それでも、どの行にも `"spans"` の中に `"request_id"` があるので、「request_id が 2 の行だけ」を集めれば、`/slow` のリクエストに何が起きたかを順番にたどれます。たくさんのリクエストを同時にさばくサーバー（UNIT 02）では、この「受付番号」が無いと、ログを読み解けません。

##### 解説2: info_span! と .instrument で、async ブロックをスパンに入れる

- `info_span!("request", request_id, method, path)` は、INFO レベルの `request` という名前のスパンを作ります。変数の名前がそのままフィールドの名前になります。
- `async { … }.instrument(span)` は、「この async ブロック（Future）が poll されている間は、ずっとこのスパンの中にいる」という印を付けます（`Instrument` trait のメソッド）。`.await` で止まっていた Future が別のスレッドで続きを動かしても、スパンは付いてきます。
- `#[instrument]` は、関数に対して同じことを自動でやってくれる書き方です。関数ではなく、その場の async ブロックをスパンに入れたいときは、`.instrument(span)` を使います。

##### 解説3: 1リクエスト1行のアクセスログと、遅いリクエストの WARN

- 処理の最後に、`status` と `elapsed_ms` をフィールドにした1行を出しています。これがアクセスログです。「どの URL に・何が返って・何ミリ秒かかったか」が、リクエストごとに1行で残ります。
- 200 ミリ秒以上かかったものは WARN にしています。ふだんは WARN 以上だけを見ていれば、遅いリクエストに気づけます。「遅い」の線をいくつにするかは、アプリの目標（例: 画面は 200 ミリ秒以内に返したい）から決めます。

#### 例9-5: メトリクスを数えて、決まった形で書き出す

> **▼ このコードがやること（先に日本語で）:** リクエストの数・エラーの数・応答時間の分布（50 ミリ秒未満・200 ミリ秒未満・それ以上）を数える「数え係」を作ります。番号によって速さと成否が変わる20個のリクエストを同時に処理して数え、最後に、メトリクスを集めるソフト（Prometheus）が読める文字の形で書き出します。

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

// アプリ全体で共有する「数え係」。数を足すだけなので、Mutex ではなく原子的な整数を使う
#[derive(Default)]
struct Metrics {
    requests: AtomicU64,
    errors: AtomicU64,
    // 応答時間の分布（ヒストグラム）: 50ms 未満 / 200ms 未満 / それ以上
    fast: AtomicU64,
    normal: AtomicU64,
    slow: AtomicU64,
}

impl Metrics {
    fn record(&self, elapsed: Duration, ok: bool) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        if !ok {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
        let bucket = if elapsed < Duration::from_millis(50) {
            &self.fast
        } else if elapsed < Duration::from_millis(200) {
            &self.normal
        } else {
            &self.slow
        };
        bucket.fetch_add(1, Ordering::Relaxed);
    }

    // Prometheus（メトリクスを集める定番のソフト）が読める文字の形で書き出す
    fn render(&self) -> String {
        format!(
            "http_requests_total {}\nhttp_errors_total {}\nhttp_duration_bucket{{le=\"0.05\"}} {}\nhttp_duration_bucket{{le=\"0.2\"}} {}\nhttp_duration_bucket{{le=\"+Inf\"}} {}",
            self.requests.load(Ordering::Relaxed),
            self.errors.load(Ordering::Relaxed),
            self.fast.load(Ordering::Relaxed),
            self.fast.load(Ordering::Relaxed) + self.normal.load(Ordering::Relaxed),
            self.requests.load(Ordering::Relaxed),
        )
    }
}

// リクエストの処理の代わり。番号によって速さと成否が変わる
async fn handle(n: u64) -> bool {
    let millis = [10, 30, 120, 250][(n % 4) as usize];
    tokio::time::sleep(Duration::from_millis(millis)).await;
    !n.is_multiple_of(5)
}

#[tokio::main]
async fn main() {
    let metrics = Arc::new(Metrics::default());
    let mut handles = Vec::new();
    for n in 1..=20 {
        let metrics = Arc::clone(&metrics);
        handles.push(tokio::spawn(async move {
            let start = Instant::now();
            let ok = handle(n).await;
            metrics.record(start.elapsed(), ok);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("{}", metrics.render());
}
```

`src/bin/u09_metrics.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_metrics
```

**▼ 実行結果**

```text
http_requests_total 20
http_errors_total 4
http_duration_bucket{le="0.05"} 10
http_duration_bucket{le="0.2"} 15
http_duration_bucket{le="+Inf"} 20
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-093" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="14" width="220" height="170" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="40" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ログ</text>
<text x="120" y="60" font-size="11" fill="#475569" text-anchor="middle">1件ずつの出来事</text>
<text x="120" y="90" font-size="11" fill="#0f172a" text-anchor="middle">「何が起きたか」</text>
<text x="120" y="114" font-size="10" fill="#0f172a" text-anchor="middle">ERROR … DB に接続できません</text>
<text x="120" y="134" font-size="10" fill="#0f172a" text-anchor="middle">WARN 遅いリクエスト 292ms</text>
<text x="120" y="164" font-size="10" fill="#475569" text-anchor="middle">例9-1・例9-3・例9-4</text>
<rect x="250" y="14" width="220" height="170" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="40" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">メトリクス</text>
<text x="360" y="60" font-size="11" fill="#475569" text-anchor="middle">数の集計</text>
<text x="360" y="90" font-size="11" fill="#0f172a" text-anchor="middle">「どれくらい起きているか」</text>
<text x="360" y="114" font-size="10" fill="#0f172a" text-anchor="middle">リクエスト 20・エラー 4</text>
<text x="360" y="134" font-size="10" fill="#0f172a" text-anchor="middle">200ms 未満が 15 件</text>
<text x="360" y="164" font-size="10" fill="#475569" text-anchor="middle">例9-5</text>
<rect x="490" y="14" width="220" height="170" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="40" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">トレース（スパン）</text>
<text x="600" y="60" font-size="11" fill="#475569" text-anchor="middle">1つのリクエストの道のり</text>
<text x="600" y="90" font-size="11" fill="#0f172a" text-anchor="middle">「どこで時間がかかったか」</text>
<text x="600" y="114" font-size="10" fill="#0f172a" text-anchor="middle">handle_request → search_notes</text>
<text x="600" y="134" font-size="10" fill="#0f172a" text-anchor="middle">idle 84.6ms</text>
<text x="600" y="164" font-size="10" fill="#475569" text-anchor="middle">例9-2</text>
<text x="360" y="214" font-size="12" fill="#475569" text-anchor="middle">メトリクスで「おかしい」に気づき、ログとスパンで「何が・どこで」を調べる</text>
<text x="360" y="236" font-size="12" fill="#475569" text-anchor="middle">3つは request_id や時刻でつなげて読む</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: ログ・メトリクス・トレースの役割の違い。メトリクスで異常に気づき、ログとスパンで原因をたどる</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 数を足すだけなら、atomic 型で十分

- `AtomicU64` は、UNIT 05 の練習問題3のヒントで紹介した、鍵なしで安全に数を書き換えられる整数です。`fetch_add(1, Ordering::Relaxed)` で、ほかのタスクに割り込まれずに1を足します。
- `Ordering::Relaxed` は、「この数を足すことだけが正しく行われればよく、ほかの変数との順番は気にしない」という指定です。単純に数を数えるだけなら、これで十分です（くわしい意味は、この本の範囲を超えるので扱いません）。
- 20個のタスクが同時に `record` を呼んでも、数え漏れはありません。`requests` は 20 になっています。

##### 解説2: 応答時間は「平均」ではなく「分布」で見る

- `fast`（50 ミリ秒未満）・`normal`（200 ミリ秒未満）・`slow`（それ以上）の3つの箱に分けて数えています。このような数え方を**ヒストグラム**と呼びます。
- 平均だけを見ると、「ほとんどは速いが、一部がとても遅い」ことに気づけません。「200 ミリ秒未満が 20 件中 15 件（75%）」のように分布で見ると、遅い利用者がどれくらいいるかが分かります。
- 書き出しの `http_duration_bucket{le="0.2"} 15` は、Prometheus の決まりで「0.2 秒以下（`le`）のものが累計 15 件」という意味です。`le="+Inf"` は「全部」で、リクエストの総数と同じになります。

##### 解説3: メトリクスは、集めて、グラフにして見る

- 本物のサーバーでは、このような数を `/metrics` のような URL で書き出し、Prometheus などのソフトが数十秒ごとに読みに来て、時間ごとのグラフにします。「エラーの数が急に増えた」「遅いリクエストの割合が上がった」に、グラフで気づけます。
- 実際のアプリでは、自分で数え係を書く代わりに、metrics や prometheus のようなクレートを使うのがふつうです。この例は、中で何が数えられているのかを知るための、手作りの版です。

> **ヒント:** メトリクスには、`user_id` のような「値の種類がとても多いもの」を入れないようにします。利用者の数だけ数え係の箱が増え、メモリも集める側の負担も増えるからです。利用者ごとの調査は、ログ（フィールドの `user_id`）でします。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: subscriber を登録し忘れて、ログが1行も出ない

```rust
use tracing::{error, info};

fn main() {
    // 受け取り係（subscriber）を登録し忘れた
    info!("起動しました");
    error!("DB に接続できません");
    println!("main の最後まで来ました");
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target\debug\e09_no_subscriber.exe`
main の最後まで来ました
```

- **何が起きたか:** `println!` の行だけが表示され、`info!` も `error!` も1行も出ていません。エラーも警告もありません。
- tracing は、ログを**出す側**と**受け取る側**が分かれています（図の上の段）。受け取り係が1人もいないと、ログは作られても、だれにも受け取られずに捨てられます。ERROR のログでさえ、です。
- **直し方:** `main` の最初で `tracing_subscriber::fmt().init();` のように受け取り係を登録します。「ログが出ない」と思ったら、まず登録を確かめましょう。

#### エラー2: subscriber を2回登録して、panic する

```rust
fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("1回目の登録");
    // 別の場所（テストの準備など）で、もう一度登録してしまう
    tracing_subscriber::fmt().json().init();
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target\debug\e09_init_twice.exe`
2026-10-24T04:44:31.190025Z  INFO e09_init_twice: 1回目の登録

thread 'main' (59120) panicked at ...\tracing-subscriber-0.3.23\src\fmt\mod.rs:520:14:
Unable to install global subscriber: SetGlobalDefaultError("a global default trace dispatcher has already been set")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\e09_init_twice.exe` (exit code: 101)
```

- **何が起きたか:** 「全体の受け取り係を登録できません。全体の受け取り係は、もう設定されています」という panic です。`init()` で登録する受け取り係は、プログラム全体で1人だけ、と決まっています。
- **直し方:** 登録は `main` の最初の1か所だけにします。テストのように、何回呼ばれるか分からない場所で登録したいときは、`init()` の代わりに `try_init()` を使います。2回目以降は panic せずに `Err` を返すので、`let _ = tracing_subscriber::fmt().try_init();` のように、失敗を無視できます。

#### エラー3: #[instrument] で、Debug の無い引数を記録しようとする（E0277）

```rust
use tracing::instrument;

// Debug を実装していない型
struct Password(String);

#[instrument]
async fn login(email: &str, password: Password) -> bool {
    !password.0.is_empty() && !email.is_empty()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    login("a@example.com", Password(String::from("secret"))).await;
}
```

同じ形のエラーが2つ出ます。1つ目を載せます。

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0277]: `Password` doesn't implement `Debug`
   --> src\bin\e09_instrument_debug.rs:6:1
    |
  6 | #[instrument]
    | ^^^^^^^^^^^^^ the trait `Debug` is not implemented for `Password`
    |
    = note: required for `&Password` to implement `Debug`
note: required by a bound in `debug`
   --> ...\tracing-core-0.1.36\src\field.rs:379:8
    |
377 | pub fn debug<T>(t: T) -> DebugValue<T>
    |        ----- required by a bound in this function
378 | where
379 |     T: fmt::Debug,
    |        ^^^^^^^^^^ required by this bound in `debug`
    = note: this error originates in the attribute macro `instrument` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Password` with `#[derive(Debug)]`
    |
  4 + #[derive(Debug)]
  5 | struct Password(String);
    |
```

- **何が起きたか:** 「`Password` は `Debug` を実装していません」というエラーです。`#[instrument]` は引数を全部フィールドにしようとし、数や文字列のような基本の型以外の引数は、`Debug` の形（`{:?}`）で記録します。`Password` には `Debug` が無いので、記録できません。
- **`help:` のとおりに直してはいけません。** `#[derive(Debug)]` を付けると、エラーは消えますが、`password=Password("secret")` のように、**パスワードがそのままログに書き出されます**。コンパイラは「型の約束」を満たす方法を教えてくれますが、「それが安全か」までは判断してくれません。
- **直し方:** 例9-2のとおり、`#[instrument(skip(password))]` で記録から外します。RustNote の `Secret` 型（この後の「RustNote ではここで使う」）のように、`Debug` を自分で実装して中身を伏せる方法もあります。

#### エラー4（エラーが出ない間違い）: spawn したタスクで、スパンが消える

```rust
use tracing::{Instrument, info, info_span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().without_time().init();

    let span = info_span!("request", request_id = 7);
    async {
        info!("リクエストの処理を始めます");
        // ① そのまま spawn: 新しいタスクには、今のスパンが引き継がれない
        tokio::spawn(async {
            info!("① 裏でメールを送りました");
        })
        .await
        .unwrap();
        // ② in_current_span: 今のスパンを付けてから spawn する
        tokio::spawn(
            async {
                info!("② 裏でメールを送りました");
            }
            .in_current_span(),
        )
        .await
        .unwrap();
    }
    .instrument(span)
    .await;
}
```

```text
 INFO request{request_id=7}: e09_lost_span: リクエストの処理を始めます
 INFO e09_lost_span: ① 裏でメールを送りました
 INFO request{request_id=7}: e09_lost_span: ② 裏でメールを送りました
```

（`cargo run` の `Compiling ...` などの行は省いています。）

- **何が起きたか:** ①の行には `request{request_id=7}` が付いていません。`tokio::spawn` で作った新しいタスクは、作った側のスパンを**自動では引き継がない**からです。どのリクエストから頼まれたメールなのか、ログから分からなくなりました。
- **直し方:** ②のように、`spawn` に渡す Future に `.in_current_span()` を付けて、今いるスパンを持たせてから渡します。別のスパンを付けたいなら `.instrument(span)` です。`spawn_blocking`（UNIT 07）のクロージャの中でも同じで、中でスパンに入り直す必要があります。

### RustNote ではここで使う

RustNote は、tracing を使っていません。ログは `eprintln!`（標準エラー出力への `println!`）で書いています。たとえば、アクセスログ（`src/routes.rs` の `access_log`）は次のとおりです。

```rust
/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
        let status = match &result {
            Ok(response) => response.status().to_string(),
            // ページが Err(see_other(..)) で返したリダイレクトは、ここではまだ Err に見える
            Err(error) if error.downcast_ref::<SeeOther>().is_some() => "303 See Other".to_string(),
            Err(error) => format!("error ({error})"),
        };
        eprintln!(
            "{} {} -> {} ({:?})",
            method(cx),
            uri(cx).path(),
            status,
            started.elapsed()
        );
    }
    result
}
```

500 のエラーの原因は、`src/views.rs` の `log_if_internal` で、原因の連なり（例8-3と同じ考え方）を `<-` でつないで書いています。

```rust
/// 500 のときだけ、原因をサーバーのログ（標準エラー出力）に書く
fn log_if_internal(status: StatusCode, error: &Error) {
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();
        eprintln!("[error] {}", causes.join(" <- "));
    }
}
```

- 「1リクエスト1行のアクセスログ」「`/health` は出さない」「500 のときは原因の連なりを残す」という**何を残すか**の考え方は、この単元の例9-4と同じです。小さなアプリなら、これで十分に役に立ちます。
- 一方で、`eprintln!` の文字の行なので、レベルも、`request_id` も、JSON の欄もありません。同時に来たリクエストの行を見分けたり、ERROR だけを探したりするのは、文字の検索に頼ることになります。tracing に置き換えて、例9-4のように `request` のスパンに `request_id` を持たせ、アクセスログを `info!(status, elapsed_ms, …)` にするのが、次の改善の候補です（引き継ぎの改善候補にも記録しています。RustNote のコードは、まだ変えていません）。

秘密の値をログに出さない工夫は、すでに入っています。Turso のトークンを入れる `Secret` 型（`src/config.rs`）です。

```rust
pub struct Secret(String);

impl Secret {
    pub fn new(value: String) -> Secret {
        Secret(value)
    }

    /// 中身が本当に必要な場所（DB に接続するとき）だけで呼ぶ。
    /// わざと目立つ名前にして、秘密を使っている場所をすぐ見つけられるようにしている
    pub fn expose(&self) -> &str {
        &self.0
    }
}

// {:?} で表示されても中身を出さない
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}
```

`Debug` を自分で実装して、中身の代わりに `Secret(********)` と表示するようにしています。もし将来 `Config` を `#[instrument]` の引数やログのフィールドに入れても、トークンは漏れません。「よくあるエラー3」の `#[derive(Debug)]` との違いを見比べてください。

### 練習問題

**問1:** 例9-1の `u09_fmt.rs` を、PowerShell で `$env:RUST_LOG = "debug"` にしてから実行しても、`debug!` の行は出ません（`fmt().init()` は `RUST_LOG` を読まないため）。`RUST_LOG=debug` のときに DEBUG の行も出るように書き換えてください（`src/bin/u09_q1.rs`）。

**問2:** 外の翻訳 API を呼ぶ関数 `call_translation_api(user_id: i64, text: &str, token: &str)` に `#[instrument]` を付けます。トークン（`token`）はログに出さず、長さだけを `token_len` として記録してください。`main` では、トークン `"sk-very-secret-token-123"` を渡して呼び、ログを確かめます（`src/bin/u09_q2.rs`）。

**問3:** 例9-5の実行結果から、次の2つを計算してください。①エラーの割合（何パーセントがエラーだったか）、②200 ミリ秒以上かかったリクエストの数と割合。

### 解答と解説

**問1の解答:**

```rust
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

fn main() {
    // RUST_LOG 環境変数で細かさを決められるようにする。無ければ info
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("RustNote を起動しました");
    info!(user_id = 42, note_id = 7, "ノートを作成しました");
    let path = "/notes/7";
    let elapsed = std::time::Duration::from_millis(35);
    warn!(%path, ?elapsed, "応答が遅めです");
    error!(
        error = "DB に接続できません",
        "ヘルスチェックに失敗しました"
    );
    debug!("細かい調査用のログ");
}
```

`$env:RUST_LOG = "debug"` にして実行した結果です。

```text
2026-10-24T04:45:47.610216Z  INFO u09_q1: RustNote を起動しました
2026-10-24T04:45:47.612491Z  INFO u09_q1: ノートを作成しました user_id=42 note_id=7
2026-10-24T04:45:47.612524Z  WARN u09_q1: 応答が遅めです path=/notes/7 elapsed=35ms
2026-10-24T04:45:47.612548Z ERROR u09_q1: ヘルスチェックに失敗しました error="DB に接続できません"
2026-10-24T04:45:47.612568Z DEBUG u09_q1: 細かい調査用のログ
```

- 例9-3と同じ `.with_env_filter(…)` を足して、`RUST_LOG` を読むようにしました。最後の行に DEBUG が出ています。
- `tracing_subscriber::fmt()` から組み立てる書き方（builder）は、細かさの指定を自分で書く作りです。tracing-subscriber 0.3.23 のソースを読むと、`tracing_subscriber::fmt::init()`（関数のほう）は `env-filter` の機能があれば `RUST_LOG` を読むと書かれています。この PC で、`fmt().init()` の例9-1を `RUST_LOG=debug` で実行しても DEBUG が出なかったことも確かめました。どちらを使っているかで動きが違うので、`RUST_LOG` が効かないと思ったら、登録の書き方を確かめましょう。

**問2の解答:**

```rust
use std::time::Duration;

use tracing::{info, instrument};

// 外の API のトークンはログに出さない。長さだけを記録する
#[instrument(skip(token), fields(token_len = token.len()))]
async fn call_translation_api(user_id: i64, text: &str, token: &str) -> String {
    tokio::time::sleep(Duration::from_millis(20)).await;
    info!("翻訳しました");
    format!("（訳）{text}")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().without_time().init();
    let translated = call_translation_api(42, "買い物メモ", "sk-very-secret-token-123").await;
    info!(%translated, "結果");
}
```

```text
 INFO call_translation_api{user_id=42 text="買い物メモ" token_len=24}: u09_q2: 翻訳しました
 INFO u09_q2: 結果 translated=（訳）買い物メモ
```

- `skip(token)` で `token` を外し、`fields(token_len = token.len())` で長さだけを足しました。`user_id` と `text` は記録され、トークンそのものはどこにも出ていません。
- ただし、`text`（翻訳する文章）も、利用者が書いたノートの中身なら、ログに出してよいかを考える必要があります。個人の情報が入っているかもしれないからです。`#[instrument]` を付けるときは、**引数を1つずつ見て、残してよいものだけを残す**習慣をつけましょう。迷ったら `skip_all` で全部外し、必要なものだけ `fields` で足す、という書き方もあります。

**問3の解答:**

- ① エラーの割合: `http_errors_total 4` を `http_requests_total 20` で割って、4 ÷ 20 = 0.2、つまり **20%** です。
- ② 200 ミリ秒以上: 全部（`le="+Inf"` の 20）から「0.2 秒以下」（`le="0.2"` の 15）を引いて、20 − 15 = **5 件**、割合は 5 ÷ 20 = **25%** です。
- 例9-5の `handle` は、番号を4で割った余りで待ち時間を決めているので、20件のうち4件に1件（5件）が 250 ミリ秒です。計算と合っています。また、5の倍数の番号（5・10・15・20 の4件）がエラーです。
- 本物のサーバーで「エラーが 20%」「4人に1人が 200 ミリ秒以上待っている」と分かったら、すぐに原因を調べるべき状態です。メトリクスで気づき、ログ（ERROR の行）とスパン（どこの idle が長いか）で原因をたどります。

### まとめ

- tracing は、ログを出す側（`info!`・`warn!`・`error!`・`debug!`・`#[instrument]`）と受け取る側（subscriber）が分かれている。subscriber は `main` の最初に1回だけ登録し、登録しないとログは1行も出ない。
- 値はフィールド（`user_id = 42`）に分けて書き、スパンで処理の区間と親子関係を記録する。リクエストごとに `request_id` のスパンを作れば、混ざったログも見分けられる。spawn するときは `.in_current_span()` でスパンを引き継ぐ。
- パスワードやトークンは `skip` でログから外す（`#[derive(Debug)]` で直さない）。数の集計はメトリクス（atomic 型・ヒストグラム）で行い、ログ・スパンと合わせて原因をたどる。
