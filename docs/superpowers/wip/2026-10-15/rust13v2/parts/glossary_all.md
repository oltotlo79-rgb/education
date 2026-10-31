## 全巻の用語集・索引

第0巻から第13巻までの「用語ミニ辞典」に載せた用語を、1つにまとめました（926 語）。英字で始まる用語を最初にアルファベット順に（`#[…]` のような記号は飛ばして、最初の英字で）、そのあとは読みの五十音順に並べています。

- **意味**は、その用語を、いちばん前の巻で説明したときの言葉です。
- **出てくる所**は、その用語を用語ミニ辞典に載せた巻と単元です。索引として、くわしい説明を探すときに使います。
- 同じ用語が、巻によって少し違う言葉で説明されていることがあります。迷ったら、出てくる所の、いちばん後ろの巻を読みます。

### 記号・数字

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `$( … )` | runtime の式。サーバーで最初の値を作り、ブラウザでも同じ計算をする | 第6巻 UNIT 09 |
| `()` | 「意味のある値がない」ことを表す値と型 | 第2巻 UNIT 03 |
| `=` での固定 | `topcoat = "=0.11.0"` のように、その版だけを使わせる書き方 | 第9巻 UNIT 05 |
| `?` 演算子 | `Err` / `None` ならすぐ返し、成功なら中身を取り出す | 第2巻 UNIT 09 |
| `^`（キャレット） | `=` を付けないときの決まり。`"0.11.0"` は 0.11.0 以上 0.12.0 未満 | 第9巻 UNIT 05 |
| 0.0.0.0 | 「すべての入口で待つ」という待ち受け側の指定 | 第0巻 UNIT 06、第13巻 UNIT 10 |
| `0xAA64`・`0x8664` | 実行ファイルの中の、CPU の種類の印。ARM64 と x64 | 第1巻 UNIT 05 |
| `1 - done` | 0 を 1 に、1 を 0 にする計算。SQL の中で done を反転する | 第10巻 UNIT 05 |
| `100 Continue` | 大きな本文を送る前の「送ってよいですか」に、サーバーが「どうぞ」と答える返事 | 第6巻 UNIT 06 |
| 127.0.0.1（localhost） | 自分の PC を指す特別な住所 | 第0巻 UNIT 06、第6巻 UNIT 01 |
| 1文の SQL | 読むと書くを1つの `UPDATE` の中で済ませること。間に、ほかの処理が入れない | 第10巻 UNIT 05 |
| 201 Created | 「新しく作りました」という返事 | 第6巻 UNIT 06 |
| 2段階借用 | `v.push(v.len())` を通す仕組み。可変借用を予約しておく | 第3巻 UNIT 07 |
| 303 See Other | 「別の URL を GET で見てください」という返事。POST の後のリダイレクトに使う | 第9巻 UNIT 02、第10巻 UNIT 02 |
| 307 Temporary Redirect | 「今は、こちらへ。同じメソッドで送り直して」という返事。`redirect(…)` が返す | 第6巻 UNIT 10 |
| 308 Permanent Redirect | 「ずっと、こちらに移りました。同じメソッドで送り直してください」という返事 | 第6巻 UNIT 03 |
| 400 Bad Request | 送られてきた内容に問題がある、という返事。RustNote は、入力を残した画面と一緒に返す | 第10巻 UNIT 02 |
| 401 Unauthorized | 誰かを確かめられなかった、という返事。ログインの失敗で使う | 第8巻 UNIT 04 |
| 403 Forbidden | 「その要求は受け付けない」という返事。Topcoat は、ほかのサイトからの POST にこれを返す | 第8巻 UNIT 01、第10巻 UNIT 03 |
| 404 Not Found | その URL のものは無い、という返事。RustNote は他人のノートにも使う | 第10巻 UNIT 03 |
| 405 Method Not Allowed | その URL は、そのメソッドでは受け付けないという返事。`allow` ヘッダーで使えるメソッドを教える | 第9巻 UNIT 02、第10巻 UNIT 06、第11巻 UNIT 06 |
| 409 Conflict | 今のデータとぶつかる、という返事。登録済みのメールで使う | 第8巻 UNIT 03、第10巻 UNIT 04 |
| 413 Payload Too Large | 本文が大きすぎて受け取れない、という返事。Topcoat の既定の上限は 2 MiB | 第6巻 UNIT 06 |
| 429 Too Many Requests | 短い時間に送りすぎた、という返事。回数の制限に引っかかったときに返る | 第8巻 UNIT 08 |
| 500 Internal Server Error | リクエストの処理の途中で、思っていなかった失敗が起きた、という状態コード | 第9巻 UNIT 10 |
| 503 | Service Unavailable。今は仕事ができない、という状態コード | 第12巻 UNIT 06 |
| 503 Service Unavailable | サーバーは返事ができるが、今は仕事を受けられない、という状態コード | 第9巻 UNIT 10 |

### A

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| abort | アボート。タスクを取り消すこと。次の `.await` の区切りで止まる | 第5巻 UNIT 02 |
| `Accept-Encoding` | ブラウザが、読める圧縮の方式を伝える見出し | 第12巻 UNIT 03 |
| ACID | transaction が守る4つの性質（全部か無しか・制約の維持・分離・永続） | 第7巻 UNIT 05 |
| `Allow` | 405 の応答で、使えるメソッドを教えるヘッダー | 第6巻 UNIT 01 |
| `ALTER TABLE … ADD COLUMN` | 表に列を足す SQL。古い版のコードはその列を知らなくても動く | 第12巻 UNIT 07 |
| `AND` と `OR` の結び付き | SQL では AND が先に結び付く。OR を含む条件はかっこで囲む | 第8巻 UNIT 07 |
| anyhow | どんなエラーでも入れられ、文脈を積み重ねて報告するクレート（1.0.104） | 第4巻 UNIT 05 |
| `anyhow::Context` | エラーに「何をしようとして失敗したか」を付ける。main の `Error:`・`Caused by:` の表示になる | 第13巻 UNIT 03 |
| API | プログラムから使うための窓口。ここでは JSON を返す URL | 第6巻 UNIT 06 |
| `app_context::<T>(cx)` | 登録した値を借りる関数。登録していない型だと panic する | 第6巻 UNIT 07 |
| `AppError` | RustNote の、利用者への返事を決める分類。HTTP の状態コードに対応する | 第13巻 UNIT 04 |
| `AppState` | Web サーバーの全ハンドラで共有する状態。`Arc` で包んで配る | 第3巻 UNIT 06・10 |
| `Arc<dyn Trait + Send + Sync>` | 実装を実行のときに選び、複数のスレッドで共有して使うための形 | 第13巻 UNIT 05 |
| `Arc<T>` | スレッドをまたいで使える、参照カウント付きの型（原子的に数える） | 第3巻 UNIT 10 |
| Argon2id | パスワード専用の、わざと遅いハッシュ関数 | 第0巻 UNIT 08、第5巻 UNIT 07、第8巻 UNIT 02、第13巻 UNIT 06 |
| `aria-current="page"` | 画面の読み上げソフトに「これが今のページです」と伝える属性 | 第6巻 UNIT 03 |
| `aria-label` | 見える文字を置かずに、読み上げソフトに名前を伝える属性 | 第9巻 UNIT 09 |
| ARM64 | Snapdragon などの CPU の体系 | 第1巻 UNIT 01 |
| Arrange・Act・Assert | テストを、準備する・操作する・確かめるの3つに分けて書く形 | 第11巻 UNIT 02 |
| `as` | 型の強制変換。値が化けることがある | 第2巻 UNIT 02 |
| `assert_eq!` | 2つの値が等しいことを確かめる。違えば `left` と `right` を表示して panic する | 第11巻 UNIT 01 |
| `asset!` | 配るファイルを宣言するマクロ。実行ファイルには番号（ID）だけが入る | 第6巻 UNIT 08 |
| `AssetBundle::load()` | 実行ファイルの隣の `assets/` から、束を読み込む関数 | 第6巻 UNIT 08 |
| `async move` | 外の変数を借りずに、async ブロックの中へ移す書き方 | 第5巻 UNIT 02 |
| async（非同期処理） | エイシンク。待ち時間の間に、ほかの仕事を進められるように書いた処理 | 第5巻 巻頭・UNIT 01 |
| `#[async_trait]` | `async fn` を持つ trait を `dyn` で使えるように書き換えるマクロ（async-trait クレート） | 第7巻 UNIT 09、第13巻 UNIT 05 |
| atomic 型 | `AtomicU64` など。鍵を使わずに、割り込まれない1回の操作で数を書き換えられる型 | 第5巻 UNIT 05 |
| `AtomicU64` | 同時に足しても数が狂わない数の型 | 第6巻 UNIT 07 |
| `ATTACH` / `DETACH` | 別の DB ファイルを名前を付けて今の接続に横付けする／外す。ファイルが無いと空の DB を作ってしまう | 第7巻 UNIT 10 |
| `audit.toml` | cargo audit の設定。受け入れる勧告の番号を、理由つきで書く | 第11巻 UNIT 10 |
| Authorization Code フロー | 認証サービスから code を受け取り、サーバーどうしで ID トークンと交換する流れ | 第8巻 UNIT 09 |
| `auto_stop_machines` | 使われていない間、マシンを止めるか（"off"・"stop"・"suspend"） | 第12巻 UNIT 04 |
| autocommit | transaction を書かないとき、1文ずつ自動で確定すること | 第7巻 UNIT 05 |
| `autocomplete="new-password"` | ブラウザに「新しいパスワードの欄」と伝える属性。強いパスワードを作ってもらえる | 第8巻 UNIT 03 |
| AUTOINCREMENT | 消した番号を使い回さない自動採番 | 第7巻 UNIT 01 |
| `.await` | Future が終わるまで待つ印。待つ間は、ほかの仕事に席を譲る | 第5巻 UNIT 01 |
| await_holding_lock | 標準の Mutex のガードを持ったまま `.await` していることを知らせる clippy の警告 | 第5巻 UNIT 05 |
| axe | 画面のアクセシビリティを、決まった点検の項目で自動に調べる道具 | 第11巻 UNIT 05 |
| A・AAAA | DNS のレコード。名前を、IPv4・IPv6 のアドレスに向ける | 第12巻 UNIT 08 |

### B

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| backpressure | バックプレッシャー。受け手が追いつかないとき、送り手を待たせたり断ったりするしくみ | 第5巻 UNIT 04 |
| backup（写し） | DB のある瞬間の状態を、別のファイルや場所に残したもの。写しの後に書いたデータは入らない | 第7巻 UNIT 06・10 |
| Base64 | バイトの並びを、英数字と `+`・`/` で表す書き方。salt とハッシュは、これで文字にしてある | 第8巻 UNIT 02 |
| Better Auth | TypeScript の認証のライブラリ。JavaScript の実行環境で動き、プラグインで機能を足せる | 第8巻 UNIT 08 |
| `BETTER_AUTH_SECRET` | Better Auth の秘密の鍵を渡す環境変数。本番で無いと止まる | 第8巻 UNIT 08 |
| bind | プレースホルダー（`?1` など）に、値を別に渡して当てはめること | 第7巻 UNIT 07、第9巻 UNIT 03 |
| bind / listen | ポートを借りる届け出／来客を待つこと | 第0巻 UNIT 06 |
| `black_box` | 値を、コンパイラーに「分からないもの」と思わせ、計算を省かせない印 | 第11巻 UNIT 08 |
| `block_on` | Future が終わるまで、今のスレッドで待ち続ける関数。async の世界の入口で使う | 第5巻 UNIT 02 |
| blocking | ブロッキング。終わるまでスレッドを手放さずに占有し続けること | 第5巻 UNIT 07 |
| blocking スレッド | `spawn_blocking` の仕事のために tokio が用意するスレッド。既定の上限は 512 本 | 第5巻 UNIT 07 |
| `BodyLimit` | URL ごとに、受け取る本文の大きさの上限を変えるレイヤー | 第6巻 UNIT 06 |
| BOM | ボム。ファイルの先頭の `EF BB BF` の印で、UTF-8 であることを知らせる | 第1巻 UNIT 01・03 |
| `bool` | `true` か `false` | 第2巻 UNIT 02 |
| bounded / unbounded | 箱に入る数の上限がある／無い channel。この巻では上限つきを使う | 第5巻 UNIT 04 |
| `box-sizing: border-box` | 幅に、内側の余白と線を含めて数える指定 | 第13巻 UNIT 09 |
| `Box<T>` | 値をヒープに置き、その持ち主になる 8 バイトの型 | 第3巻 UNIT 08 |
| `/Brepro` | Windows のリンカーに、時刻の代わりに中身から計算した値を使わせる指定 | 第12巻 UNIT 01 |
| broadcast | 送った1件を、受け手の全員に届ける channel。遅れた受け手は読み飛ばす | 第5巻 UNIT 04 |
| `BTreeMap` | キーの順に並ぶ表 | 第2巻 UNIT 10 |
| Build Tools for Visual Studio | Visual Studio を開かずに、MSVC の道具だけを入れる製品 | 第1巻 UNIT 05 |
| Builder / Database / Connection | libsql で、DB の開き方を決める部品／DB そのもの／DB と話す窓口 | 第7巻 UNIT 07 |
| `busy_timeout` | ほかの接続が書き込み中のとき、何ミリ秒まで待つか。接続ごとの設定 | 第13巻 UNIT 02 |

### C

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `cache-control` | どれだけの間、覚えてよいかを伝えるヘッダー。`immutable` は「中身は変わらない」 | 第6巻 UNIT 08 |
| `CACHED` | docker build の記録で、その手順の結果を前のビルドから使い回したという印 | 第12巻 UNIT 02、第13巻 UNIT 10 |
| CancellationToken | 停止の合図を配る札。`child_token` で子の札を作り、`cancel` で取り消す | 第5巻 UNIT 06 |
| capacity | `Vec` が引っ越さずに入れられる数 | 第2巻 UNIT 06 |
| Cargo | Rust のプロジェクト管理の道具。ビルド・実行・テストなどを取り仕切る | 第0巻 UNIT 04 |
| cargo | カーゴ。Rust のビルドと部品（クレート）の管理をする道具 | 第1巻 UNIT 04 |
| `cargo check` | 型と文法だけを確かめる。実行ファイルは作らないので速い | 第1巻 UNIT 06 |
| `cargo clippy` | よくある書き方の注意を出す。`-- -D warnings` で注意をエラーにする | 第1巻 UNIT 06 |
| `cargo fmt` | rustfmt で、字下げや空白を決まった形に整える。`--check` は差分を見せるだけ | 第1巻 UNIT 06 |
| `cargo install` | crates.io からソースを取ってきてビルドし、`.cargo\bin` に置く | 第1巻 UNIT 08 |
| `cargo run --example` | `examples/` に置いた、アプリとは別の小さな実行ファイルを動かすコマンド | 第9巻 UNIT 10 |
| `cargo test` | テストを動かして、結果が期待どおりかを確かめる | 第1巻 UNIT 06 |
| `cargo test --test 名前` | `tests/名前.rs` のテストだけを実行する。後ろに言葉を足すと、名前で絞り込む | 第9巻 UNIT 01 |
| `cargo tree` | 依存の木を表示するコマンド。`-i` は逆向き、`-d` は2つ以上の版、`-e features` は機能 | 第9巻 UNIT 05、第13巻 UNIT 01 |
| `cargo tree -i` | ある部品を、どの部品が使っているかを、逆向きにたどる | 第11巻 UNIT 10 |
| `cargo update -p` | 選んだクレート（と、それが使うもの）だけを、新しい版に上げる | 第9巻 UNIT 05 |
| cargo-audit | Cargo.lock の部品を、RustSec と照らし合わせる道具 | 第11巻 UNIT 10 |
| cargo-fuzz | libFuzzer を使う Rust の fuzz の道具。nightly で動く | 第11巻 UNIT 04 |
| cargo-mutants | Rust の関数や記号を機械的に書き換えて、ミューテーションテストをする道具 | 第11巻 UNIT 06 |
| `Cargo.lock` | 実際に使う依存の版とチェックサムの記録。アプリでは Git に保存する | 第4巻 UNIT 07、第9巻 UNIT 05、第13巻 UNIT 01 |
| `Cargo.toml` | プロジェクトの表紙。名前・版・使う部品を書く | 第1巻 UNIT 04、第13巻 UNIT 01 |
| caught・missed・unviable | 倒した・見逃した・組み立てられなかった、の3つの結果 | 第11巻 UNIT 06 |
| `Caused by:` | anyhow のエラーの表示で、元の原因を並べる見出し | 第9巻 UNIT 06 |
| `Cell<T>` | Copy な値を `get` / `set` で出し入れするだけの、内部可変性の型 | 第3巻 UNIT 09 |
| `cfg!(debug_assertions)` | debug ビルドなら `true`、release ビルドなら `false` になる式 | 第12巻 UNIT 01 |
| `#[cfg(test)]` | `cargo test` のときだけコンパイルする印。本番の実行ファイルには入らない | 第11巻 UNIT 01 |
| channel | チャネル。タスクの間で値を渡す通り道。送る口と受け取る口の1組 | 第5巻 UNIT 04 |
| `char` | Unicode の1文字。4バイト | 第2巻 UNIT 02 |
| CHECK 制約 | 列に入れてよい値の決まり。破る値は DB がエラーにする（`CHECK constraint failed`） | 第10巻 UNIT 05 |
| `checked_add` / `checked_mul` | あふれたら `None` を返す計算。あふれたことをプログラムで扱える | 第12巻 UNIT 01 |
| `checked_mul` | 掛け算があふれるときに `None` を返すメソッド。あふれる値を断るのに使う | 第6巻 UNIT 09 |
| checksum | クレートの中身から計算した指紋（SHA-256）。取ってきた中身が記録と同じかを確かめる | 第13巻 UNIT 01 |
| `Child` | コンポーネントが受け取る中身。呼び出しの引数のあとに書いた部品が入る | 第6巻 UNIT 05 |
| CI | 変更のたびに、決まった確かめを自動で毎回動かすこと | 第11巻 UNIT 10、第13巻 UNIT 10 |
| claim | 仕事を取り出して自分のものにすること。1つの `UPDATE … RETURNING` で行う | 第5巻 UNIT 10 |
| `class!` | 条件に合うときだけクラスを足して、`class` の値を作るマクロ | 第6巻 UNIT 05、第10巻 UNIT 01 |
| `@click` | クリックされたときに動く、ブラウザの中の処理（イベントハンドラー） | 第6巻 UNIT 09 |
| `client_ip` | リクエストを送ってきた相手のアドレスを返す Topcoat の関数。プロキシの後ろでは `TrustedProxies` の設定が要る | 第8巻 UNIT 10 |
| clippy | クリッピー。Rust のよくある書き間違いや、もったいない書き方を指摘する道具 | 第1巻 UNIT 03 |
| Clone / clone() | 明示的に本体ごと複製すること。費用がかかることがある | 第3巻 UNIT 02 |
| CNAME | DNS のレコードの1つ。名前を、別の名前に向ける（サブドメインに使う） | 第12巻 UNIT 08 |
| `COALESCE` | 最初の `NULL` でない値を返す関数。`COALESCE(SUM(done), 0)` で `NULL` を 0 にする | 第7巻 UNIT 09 |
| COMMIT / ROLLBACK | transaction の変更を確定する／全部取り消す | 第7巻 UNIT 05 |
| `common::test_db()` | テストごとに一時フォルダを作り、マイグレーション済みの空の DB を渡す RustNote の関数 | 第11巻 UNIT 02 |
| `#[component]` | `view!` の部品を作る印。RustNote の `notes_index` は、中で DB を読む async の部品 | 第10巻 UNIT 01、第13巻 UNIT 07 |
| `Config` | 環境変数から起動のときに1回だけ作る、RustNote の設定 | 第9巻 UNIT 06、第13巻 UNIT 03 |
| `content-encoding: gzip` | 本文を gzip で圧縮してある、という印 | 第6巻 UNIT 02 |
| `Content-Length` | 本文のバイト数。受け取る側は、この数だけ本文を読む | 第6巻 UNIT 01 |
| `Content-Type` | 本文の種類を表すヘッダー。`Json` は `application/json` でなければ断る | 第6巻 UNIT 06 |
| `.context(…)` | anyhow で、エラーに「何をしようとして失敗したか」の文を付けるメソッド | 第9巻 UNIT 06 |
| Cookie | サーバーがブラウザに預ける小さなメモ。リクエストのたびに送り返される | 第0巻 UNIT 08 |
| Copy | 書き写すだけで複製できる型の印。代入しても元が使える | 第3巻 UNIT 02 |
| `core.autocrlf` | Git が改行を自動で変換する設定。Git for Windows の既定は `true` | 第1巻 UNIT 02 |
| coroutine | コルーチン。コンパイラの中での、async ブロックの呼び名。エラーメッセージに出てくる | 第5巻 UNIT 02 |
| COVERING INDEX | 索引だけで答えが出せたので、表を見に行かなかったこと | 第7巻 UNIT 04 |
| CPU | 命令を実行して計算する部品 | 第0巻 UNIT 01 |
| CPU の種類（アーキテクチャ） | CPU の命令の体系。Windows の PC では x64 と ARM64 が主 | 第1巻 巻頭・UNIT 01 |
| CPU バウンド / I/O バウンド | 計算そのものに時間がかかる処理／返事を待つ時間がほとんどの処理 | 第5巻 UNIT 07 |
| crate | クレート。コンパイルの単位（ライブラリ・実行ファイル） | 第4巻 UNIT 02・06 |
| `crate::` | 自分のクレートの根元から道のりを書くときの始まり。ライブラリの外（`main.rs`・`tests/`・`examples/`）からは `rustnote::` を使う | 第9巻 UNIT 04 |
| criterion | 統計を使って速さを測る、Rust のベンチマークのクレート | 第11巻 UNIT 08 |
| CRLF / LF | Windows の流儀の改行（CR+LF）と、Linux・macOS の流儀の改行（LF） | 第1巻 UNIT 02 |
| CRUD | Create（作る）・Read（読む）・Update（直す）・Delete（消す）。データを扱う画面の基本の4つの操作 | 第10巻 UNIT 01、第13巻 UNIT 08 |
| CSRF | ほかのサイトのページから、ログイン中の人のブラウザに勝手に要求を送らせる攻撃 | 第6巻 UNIT 10、第8巻 UNIT 01、第8巻 UNIT 06 |
| CSS の変数 | `--color-text: #0f172a;` のように決め、`var(--color-text)` で使う値。RustNote は色を `:root` にまとめる | 第13巻 UNIT 09 |
| CSS セレクター | `ul.note-list li a` のように、HTML の要素を場所で指定する書き方 | 第11巻 UNIT 03 |
| curl | HTTP のリクエストを送り、結果を文字で見せる道具。Windows では `curl.exe` | 第0巻 UNIT 05 |
| curl の `(7)` | サーバーに接続できなかったときの curl のエラー | 第6巻 UNIT 01 |
| cursor（keyset pagination） | 前のページの最後の行の値を続きの印にして、その後ろを読むページ送り | 第10巻 UNIT 08 |
| `Cx` | 今の要求の中身と、アプリで共有する値を持つ入れ物。`cx: &Cx` で受け取る | 第6巻 UNIT 04 |

### D

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| database is locked | ほかの接続が書き込みの鍵を持っているときのエラー | 第7巻 UNIT 05、第11巻 UNIT 02 |
| `DATABASE_URL` | RustNote の DB のつなぎ先。ファイルの名前・`http://`・`libsql://` | 第1巻 UNIT 09 |
| `datetime('now')` | SQLite の今の日時（UTC、秒まで）。`updated_at` に入れる | 第10巻 UNIT 04 |
| dead-letter | デッドレター。上限までやり直しても成功しなかった仕事の置き場。人が確かめて入れ直す | 第5巻 UNIT 10 |
| debug ビルド | `cargo build` で作る試し用の実行ファイル。最適化なし、整数のあふれを確かめる。`target\debug` にできる | 第12巻 UNIT 01 |
| `default-run` | 実行ファイルが2つ以上あるとき、`cargo run` で動かすものを決める `Cargo.toml` の設定 | 第6巻 UNIT 02 |
| `#[default]` | 呼ぶ側が省いた引数に、既定の値を使う印 | 第6巻 UNIT 05 |
| `DELETE FROM sessions` | sessions 表の行を全部消し、全員のセッションをサーバーの側で一度に終わらせる SQL | 第8巻 UNIT 10 |
| `delete_expired` | 期限切れのセッションの行を消す関数。RustNote はログインのときに呼ぶ | 第8巻 UNIT 05 |
| `derive` | `Debug` などの機能を自動で付ける属性 | 第2巻 UNIT 07 |
| `Deserialize` / `Serialize` | serde の trait。本文から構造体を作る／構造体から JSON を作る | 第6巻 UNIT 06 |
| `<details>` | 開いたり閉じたりできる HTML の部品。RustNote は作成のフォームを入れる | 第13巻 UNIT 08 |
| `[dev-dependencies]` | テスト・examples でだけ使うクレート。本番の実行ファイルには入らない | 第13巻 UNIT 01 |
| `discover()` | プログラムの中の `#[page]` の関数を、全部自動で集める（`RouterBuilderDiscoverExt` を use して使う） | 第6巻 UNIT 02 |
| `Display` | `{}` で表示する方法を決めるトレイト | 第2巻 UNIT 09 |
| `div_ceil` | 割り算の切り上げ。55 件を 20 件ずつなら 3 ページ | 第10巻 UNIT 08 |
| DNS | ホスト名を IP アドレスに引き直すしくみ | 第0巻 UNIT 05 |
| Docker Desktop | Windows でコンテナを動かすアプリ。WSL2 の上で動く | 第1巻 UNIT 07 |
| `docker history` | イメージのレイヤーと、それぞれの大きさ・作った手順を並べて見るコマンド | 第12巻 UNIT 02 |
| `docker image inspect` | イメージの設定（環境変数・利用者・ポートなど）を見るコマンド | 第12巻 UNIT 05 |
| `.dockerignore` | docker build のときに、Docker へ送らないファイルの一覧 | 第13巻 UNIT 10 |
| doctest | ドキュメントの `# Examples` のコードを、テストとして実行する仕組み | 第4巻 UNIT 10 |
| `dotenvy::dotenv()` | `.env` を読み、まだ無い名前の環境変数だけを足す関数 | 第9巻 UNIT 06 |
| `downcast_ref` | エラーが、ある型（`NotFoundError` など）かどうかを確かめて、その型として借りるメソッド | 第6巻 UNIT 10 |
| Drop | 値が片付けられること。`Drop` トレイトで片付け時の処理を書ける | 第3巻 UNIT 01 |
| `--dry-run` | 本当には変えずに、何が変わるかだけを表示する指定 | 第9巻 UNIT 05 |
| DTO | Data Transfer Object。外とのやり取りのためだけの入れ物の型 | 第4巻 UNIT 08 |
| `DUMMY_HASH` | 登録が無いときにも照合して、時間をそろえるためのダミーのハッシュ | 第8巻 UNIT 04 |
| dyn compatible | `dyn` で使える条件を満たす trait。旧称「オブジェクト安全」 | 第4巻 UNIT 04、第7巻 UNIT 09 |

### E

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| E0046 | trait の約束のメソッドが、`impl` に全部はそろっていないエラー | 第9巻 UNIT 04 |
| E0061 | メソッドや関数に渡した引数の数が違うエラー | 第8巻 UNIT 01、第8巻 UNIT 07 |
| E0277 | 必要な trait が実装されていないエラー（ここでは `view!` に入れられない型） | 第6巻 UNIT 04、第10巻 UNIT 03 |
| E0308 | 型が合わないエラー（ここでは `UserId` の場所に `NoteId` を渡した） | 第8巻 UNIT 01、第10巻 UNIT 02、第10巻 UNIT 04 |
| E0382 | 移動した（持ち主が変わった）値を、もう一度使おうとしたエラー | 第6巻 UNIT 05、第10巻 UNIT 01 |
| E0423 | 中身が private なタプル構造体を、外から直接作ろうとしたときのエラー | 第9巻 UNIT 03 |
| E0432 | `use` で読み込もうとしたものが見つからないエラー | 第8巻 UNIT 02、第9巻 UNIT 04 |
| E0433 | 名前が見つからないエラー（ここでは `tokio::main`） | 第9巻 UNIT 05、第13巻 UNIT 01 |
| E0507 | 借りているだけの値から、中身を移動しようとしたエラー | 第10巻 UNIT 01 |
| E0573 | 型が来るはずの所で、型が見つからないエラー（ここでは `#[page]` を付け忘れた） | 第6巻 UNIT 03 |
| E0609 | その型に、その名前の項目（フィールド）が無いエラー | 第8巻 UNIT 05 |
| E0752 | `main` 関数を `async` にしたエラー | 第9巻 UNIT 05、第13巻 UNIT 01 |
| E2E | 利用者と同じ入口（ブラウザ）から、サーバー・DB まで本物でつないで確かめるテスト | 第11巻 UNIT 05 |
| edition | Rust の文法の版。このシリーズは 2024 | 第0巻 UNIT 04 |
| `Elapsed` | timeout の時間切れを表すエラーの型。表示は `deadline has elapsed` | 第5巻 UNIT 06 |
| `Email::parse` | メールアドレスを検査し、前後の空白を取って小文字にそろえる関数 | 第8巻 UNIT 03 |
| `.env` | 開発のときに環境変数を書いておくファイル。秘密が入るので Git に入れない | 第9巻 UNIT 06、第13巻 UNIT 03 |
| `.env` / `.env.example` | 秘密の値を書く環境変数のファイルと、その見本。Git に入れるのは見本だけ | 第1巻 UNIT 02、第4巻 UNIT 09、第13巻 UNIT 01 |
| ER 図 | 表（もの）と、表どうしのつながりを描いた図 | 第9巻 UNIT 03 |
| `error = not_found` / `error = bad_request` | 読めなかったときに、404 / 400 の返事にする指定 | 第6巻 UNIT 04 |
| `error_boundary` | 中で起きたエラーを、`fallback` の画面に変える部品 | 第6巻 UNIT 10、第13巻 UNIT 07 |
| `error_for_status` | 4xx・5xx の答えを `Err` に変える reqwest のメソッド | 第5巻 UNIT 08 |
| `error_page` | レイアウトの中で、エラーを状態コード付きの画面にする関数 | 第10巻 UNIT 09 |
| ESCAPE | LIKE の形の中で「次の1文字はただの文字」という印に使う文字を決める指定 | 第10巻 UNIT 07 |
| `ESCAPE '\'` | `LIKE` の中で、`\` のうしろの文字（`%`・`_`）を、ただの文字として扱わせる指定 | 第11巻 UNIT 02、第13巻 UNIT 05 |
| `escape_like` | キーワードの `%`・`_`・`\` の前に `\` を付ける RustNote の関数 | 第10巻 UNIT 07 |
| `examples/` | 本番の実行ファイルに入らない、小さなプログラムを置くフォルダ。`cargo run --example 名前` で動かす | 第9巻 UNIT 07 |
| execute / query | 行を返さない文を実行して変更した行数を返す／行を返す文を実行して `Rows` を返す | 第7巻 UNIT 07 |
| expand / contract | DB の形を変えるとき、まず足し（広げ）、全部のコードが新しい形になってから古い物を消す（縮める）進め方 | 第12巻 UNIT 07 |
| `expires_at` | セッションの有効期限。1970 年からの秒数で `sessions` 表に入れる | 第8巻 UNIT 05 |
| EXPLAIN QUERY PLAN | 文を実行せずに、DB がどう探すつもりか（計画）を見せる命令 | 第7巻 UNIT 04、第10巻 UNIT 08、第13巻 UNIT 02 |
| `expose()` | `Secret` の中身を取り出すメソッド。秘密を使う所を見つけやすくするための名前 | 第9巻 UNIT 06、第13巻 UNIT 03 |

### F

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `f64` | 小数の型。わずかな誤差がある | 第2巻 UNIT 02 |
| Fake | フェイク。テスト用の、本物と同じ約束を守る簡単な実装 | 第4巻 UNIT 02 |
| feature | クレートの機能のスイッチ。使わない機能を外すと、その部品も組み立てない | 第11巻 UNIT 10 |
| features | クレートの機能の ON/OFF。`#[cfg(feature = "...")]` でコードを出し入れする | 第4巻 UNIT 07、第7巻 UNIT 08、第9巻 UNIT 05、第13巻 UNIT 01 |
| `find_own_note` | id と持ち主の両方で、自分のノートを探す関数。無い・他人なら `AppError::NotFound` | 第10巻 UNIT 03 |
| `flash::set`・`flash::take` | お知らせを予約する関数と、読んで消す関数 | 第10巻 UNIT 09 |
| flex | `display: flex` で、子の要素を横（または縦）に並べるしくみ。`flex-wrap: wrap` で折り返す | 第13巻 UNIT 09 |
| `fly auth` | ログインまわりのコマンド。login・whoami・logout・signup | 第1巻 UNIT 10 |
| `fly logs` | Fly.io のマシンのログを見るコマンド | 第12巻 UNIT 08 |
| `fly secrets set` | 秘密の値を Fly.io に保管し、マシンを入れ替えて渡すコマンド（`--stage` で、登録だけ） | 第12巻 UNIT 05 |
| fly.toml | Fly.io でアプリを動かすときの設定ファイル（TOML）。名前・地域・ポート・ヘルスチェック・マシンの大きさを書く | 第12巻 UNIT 04 |
| `.fly\config.yml` | ログインしたときのトークンが保存されるファイル。人に見せない | 第1巻 UNIT 10 |
| flyctl | Fly.io を操作するコマンド。`fly` と打っても同じ | 第1巻 UNIT 10 |
| `:focus-visible` | 主にキーボードで移ったときに当たる CSS の指定。見える枠に置き換えるときに使う | 第10巻 UNIT 10 |
| `force_https` | http:// のリクエストを https:// へ移す設定 | 第12巻 UNIT 04 |
| `Form<T>` | フォームの本文（`application/x-www-form-urlencoded`）を、構造体 `T` にして受け取る引数 | 第6巻 UNIT 06 |
| `From` / `TryFrom` | 必ず成功する変換／失敗するかもしれない変換 | 第2巻 UNIT 02 |
| `#[from]` | thiserror の印。`From` を作るので、`?` でそのエラーを自動で変換できる | 第13巻 UNIT 04 |
| `FromStr` | 文字列から作れる型を表す標準の trait。実装すると `.parse()` が使える | 第4巻 UNIT 01・09、第6巻 UNIT 04、第13巻 UNIT 04 |
| Future | フューチャー。「まだ終わっていない仕事」を表す値。`.await` か poll されるまで動かない | 第5巻 UNIT 01 |
| fuzz・fuzzer | 入力を少しずつ変えて、とても多く試し、パニックを探すこと・その道具 | 第11巻 UNIT 04 |

### G

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| generics | ジェネリクス。型をパラメータとして受け取る仕組み（`fn f<T>(x: T)`） | 第4巻 UNIT 03 |
| `Get-FileHash` | ファイルの中身からハッシュを計算する PowerShell のコマンド。中身が変わったかを確かめられる | 第9巻 UNIT 07 |
| `get_all` | 同じ名前のヘッダーを、全部取り出すメソッド（`get` は1つ目だけ） | 第11巻 UNIT 03 |
| `getByLabel`・`getByRole` | ラベルや、ボタン・リンクの名前で要素を探す、Playwright の探し方 | 第11巻 UNIT 05 |
| `getComputedStyle` | ブラウザで、CSS が効いた結果の値を読む JavaScript の関数 | 第13巻 UNIT 09 |
| Git | ファイルの変更の記録を残す道具 | 第1巻 UNIT 02 |
| `.gitattributes` | ファイルの種類ごとの扱い（改行コードなど）の決まり | 第1巻 UNIT 02 |
| `.gitignore` | Git に入れないファイルの一覧 | 第1巻 UNIT 02、第13巻 UNIT 01 |
| glibc | Linux の C の基本のライブラリ。新しい glibc で作った実行ファイルは、古い glibc の上では動かないことがある | 第12巻 UNIT 02 |
| global allocator | プログラム全体の、メモリを借りる・返す係。`#[global_allocator]` で替えられる | 第11巻 UNIT 08 |
| `grace_period` | マシンが起動してから、最初のヘルスチェックまで待つ時間 | 第9巻 UNIT 10、第12巻 UNIT 06 |
| graceful shutdown | グレースフル・シャットダウン。処理中の仕事をできるだけ終わらせてから止まる止め方 | 第5巻 UNIT 06 |

### H

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `has_prev`・`has_next` | 「前へ」「次へ」を出すかどうか。ページが 1 より大きいか、ページの数より小さいか | 第10巻 UNIT 08 |
| `HashMap` | キーから値を引く表。順番は決まっていない | 第2巻 UNIT 10 |
| `:hidden` などの結び付け属性 | signal が変わるたびに、ブラウザが属性を付け直す属性 | 第6巻 UNIT 09 |
| hidden の入力欄 | 画面に見えないが、ほかの入力欄と同じように送られる `<input type="hidden">` | 第10巻 UNIT 04 |
| `$HOME` | ホームフォルダ。PowerShell が先に `C:\Users\（名前）` に直す | 第1巻 UNIT 10 |
| host | rustc が動いている PC の target。`rustc -vV` の `host:` | 第1巻 UNIT 05 |
| `__Host-` | クッキーの名前の頭に付ける決まり。`Secure`・`Path=/`・ドメインの指定なしのときだけ、ブラウザが受け付ける | 第8巻 UNIT 01、第8巻 UNIT 06 |
| `__Host-session` | Topcoat のセッションのクッキー。`__Host-` で始まる名前は、Secure と Path=/ が必須で、ドメインを指定できない | 第13巻 UNIT 06 |
| Hrana | libsql がリモートの DB とやり取りするときの通信の仕組み。リモートのエラーは `Hrana(…)` で返る | 第7巻 UNIT 07・08、第12巻 UNIT 06、第12巻 UNIT 09 |
| `href!` | `#[page]` などの関数の名前から、リンク先の URL を作るマクロ。無いページはコンパイルで見つかる | 第6巻 UNIT 03、第9巻 UNIT 04、第10巻 UNIT 03、第13巻 UNIT 07 |
| HTML | Web ページの中身を書く言葉 | 第0巻 UNIT 05 |
| HTTP | ブラウザとサーバーが、要求と応答をやり取りするときの決まり | 第6巻 UNIT 01 |
| HTTP / HTTPS | ブラウザとサーバーのやりとりの決まり。HTTPS は暗号化版 | 第0巻 UNIT 05 |
| HTTP クライアント | HTTP のリクエストを送り、答えを受け取る側のプログラム | 第5巻 UNIT 08 |
| `HttpOnly` | ページの JavaScript から読めないクッキーの設定 | 第8巻 UNIT 06 |
| `HttpOnly` / `Secure` / `SameSite=Lax` | JavaScript から読めない／HTTPS のときだけ送る／ほかのサイトからの POST には付けない、というクッキーの属性 | 第8巻 UNIT 01 |

### I

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| ID の数え上げ | ID を 1 から順に試して、答えの違いから、ほかの人のデータの有無や数を知ること | 第10巻 UNIT 03 |
| ID トークン | 認証サービスが署名した「この人は誰か」の証明。iss・aud・exp・sub などを持つ | 第8巻 UNIT 09 |
| IDOR | ID を書き換えるだけで他人のデータが見えてしまう不具合 | 第0巻 UNIT 08、第8巻 UNIT 01、第8巻 UNIT 07、第11巻 UNIT 06 |
| `#[ignore]` | ふだんは動かさないテストの印。`--ignored` で動かす | 第11巻 UNIT 06 |
| `immutable` | `cache-control` の値。中身が変わらないので、期限まで確かめ直さなくてよい、という意味 | 第13巻 UNIT 09 |
| `impl` | 型に関数（メソッドなど）を付けるブロック | 第2巻 UNIT 07 |
| `IN` | 列の値が、並べた値のどれかと一致するかを調べる条件。`?` は 32,766 個まで | 第7巻 UNIT 01・09 |
| `include_str!` | コンパイル時にファイルを読み、文字列としてプログラムに埋め込むマクロ | 第7巻 UNIT 06、第9巻 UNIT 03、第13巻 UNIT 02 |
| index（索引） | 列の値を並べ替えて、行の場所と一緒に持つ別のデータ。検索を速くする | 第7巻 UNIT 04 |
| `#[instrument]` | 関数の呼び出しをスパンにする印。引数がフィールドになる（`skip` で外せる） | 第5巻 UNIT 09 |
| `internal_port` | Fly.io の入口が、マシンの中のどのポートへリクエストを届けるか | 第12巻 UNIT 04 |
| `interval`・`timeout` | ヘルスチェックの間隔と、答えを待つ長さ。RustNote は 30 秒と 5 秒 | 第9巻 UNIT 10 |
| IP アドレス | ネットワーク上のコンピュータの住所 | 第0巻 UNIT 06 |
| `is_current` | そのリンク先を、今開いているかどうかを返すメソッド | 第6巻 UNIT 03 |

### J

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `join` | スレッドの終わりを待ち、結果を受け取ること | 第3巻 UNIT 10 |
| JOIN / LEFT JOIN | 2つの表をつないで読むこと。LEFT JOIN は相手がいない左の行も残す | 第7巻 UNIT 02 |
| `join_next` / `join_all` | JoinSet から結果を1つずつ／全部まとめて受け取るメソッド。どちらも終わった順 | 第5巻 UNIT 03 |
| JoinError | タスクが panic したか、取り消されたことを表すエラー | 第5巻 UNIT 02 |
| JoinHandle | ジョインハンドル。タスクの引換券。`.await` で結果を受け取る | 第5巻 UNIT 02 |
| JoinSet | ジョインセット。子のタスクの袋。`join_next` で終わった順に受け取り、捨てると残りを取り消す | 第5巻 UNIT 03 |
| JSON | `{"title":"milk"}` のように、名前と値の組を書くデータの形。プログラムどうしのやり取りに使う | 第6巻 UNIT 06 |
| `Json<T>` | JSON の本文を構造体 `T` にして受け取る引数。返すときは、構造体を JSON にする | 第6巻 UNIT 06 |

### K

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| kebab-case | 単語を `-` でつなぐ書き方（`blog-posts`）。モジュール名の `_` は URL では `-` になる | 第6巻 UNIT 03 |
| `#[key(…)]` | `for` で並べた1件ずつを見分ける印。DB の ID などを使う | 第6巻 UNIT 05、第13巻 UNIT 08 |

### L

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `<label for="id">` | 入力欄の名前。`for` と入力欄の `id` が同じだと、つながる | 第9巻 UNIT 09 |
| `Lagged` | broadcast で、受け手が追いつけずに読み飛ばした件数を知らせるエラー | 第5巻 UNIT 04 |
| `#[layer]` | 要求の処理を前と後ろではさむ関数。`next.run(cx, body).await` で中を動かす | 第6巻 UNIT 03、第13巻 UNIT 07 |
| `#[layout]` | いくつものページを包む共通の外枠。`(slot)` の所にページの中身が入る | 第6巻 UNIT 03、第13巻 UNIT 07 |
| `LazyLock` | 最初に使われたときに1回だけ値を作る、標準ライブラリの道具 | 第5巻 UNIT 05、第8巻 UNIT 02、第13巻 UNIT 06 |
| lease | リース。worker が仕事を取り出したときの持ち時間。過ぎるとほかの worker が取り直せる | 第5巻 UNIT 10 |
| Let's Encrypt | HTTPS の証明書を自動で無料で発行するしくみ。Fly.io が使う | 第12巻 UNIT 08 |
| libSQL | SQLite をもとにした DB。ファイルとしても、サーバー越しにも使える | 第1巻 UNIT 09、第7巻 巻頭 |
| LIKE | 文字の並びが形に合うかを調べる SQL の書き方。`%` は 0 文字以上、`_` は 1 文字の何か | 第10巻 UNIT 07 |
| `LIMIT` / `OFFSET` | 読む行の数／先頭から飛ばす行の数。RustNote は 1 ページ 20 件 | 第10巻 UNIT 01 |
| `-- --list` / `-- --nocapture` | テストの名前の一覧だけを出す／成功したテストの `println!` も表示する（UNIT 02 でも使う） | 第9巻 UNIT 01 |
| `Location` | 3xx の応答で、行き先を教えるヘッダー | 第6巻 UNIT 01 |
| `--locked` | 作った人が確かめた `Cargo.lock` のとおりの版で作る指定 | 第1巻 UNIT 08、第4巻 UNIT 07、第9巻 UNIT 05、第11巻 UNIT 10、第13巻 UNIT 01、第13巻 UNIT 10 |
| `LOGIN_FAILED` | ログインの失敗の文。どの失敗でも「メールアドレスまたはパスワードが違います」 | 第8巻 UNIT 04 |
| `&lt;` / `&gt;` / `&amp;` / `&quot;` | `<`・`>`・`&`・`"` を、ただの文字として HTML に書く書き方 | 第6巻 UNIT 05 |

### M

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `m` / `t` / `p` | Argon2 の重さの設定。メモリ（KiB）・くり返す回数・並べて動かす数 | 第8巻 UNIT 02 |
| `manifest.toml` | 束の目録。宣言の ID と、ファイル名・ハッシュ・種類の対応を書く | 第6巻 UNIT 08、第12巻 UNIT 03 |
| `match` | 値を形（パターン）ごとに振り分ける式 | 第2巻 UNIT 04 |
| `Max-Age=0` | すぐに期限が切れるクッキー。ブラウザに、そのクッキーを消させる | 第10巻 UNIT 09 |
| MAX_PATH（パスの上限） | 多くの Windows のプログラムが扱えるパスの長さの上限。259 文字まで | 第1巻 UNIT 01 |
| `#[memoize(as_ref)]` | 覚えた `Option<T>` や `Result<T, E>` の中身を、借りる形（`Option<&T>` など）で返す指定 | 第6巻 UNIT 07、第13巻 UNIT 06 |
| MFA（多要素認証） | 知っている物と持っている物など、2つ以上で本人を確かめること | 第8巻 UNIT 09 |
| `--migrate-only` | この巻で RustNote のコピーに足した、マイグレーションだけをして終わる動かし方 | 第12巻 UNIT 07 |
| migration | マイグレーション。スキーマの変更を番号付きの手順として管理し、未適用の分だけを順に適用する仕組み | 第7巻 UNIT 06 |
| `min_machines_running` | いつも動かしておくマシンの最小の数。0 なら、使われていない間は全部止まる | 第12巻 UNIT 09 |
| `/mnt/c` | Linux から見た、Windows の C ドライブ | 第1巻 UNIT 07 |
| module | モジュール。crate の中の名前の区画。公開範囲の境界 | 第4巻 UNIT 06 |
| `module_param!` | モジュールの区切りを `{id}` にし、読むための型（`Id`）を作るマクロ | 第6巻 UNIT 04、第10巻 UNIT 03、第13巻 UNIT 08 |
| move | 代入や関数呼び出しで、持ち主が移ること。元の変数は使えなくなる | 第3巻 UNIT 02 |
| `move` クロージャ | 使う外の変数を、借りずに持っていくクロージャ | 第3巻 UNIT 10 |
| mpsc | 送り手は何人でも、受け手は1人の channel（multi-producer, single-consumer） | 第5巻 UNIT 04 |
| MSRV / `rust-version` | そのクレートを使える、いちばん古い Rust の版 | 第9巻 UNIT 05 |
| MSVC | Microsoft Visual C++。Windows で実行ファイルを作る道具一式。リンカー `link.exe` が入っている | 第1巻 UNIT 05 |
| multi-stage ビルド | 1つの Dockerfile に FROM を2つ以上書き、前の段で作った物から要る物だけを COPY --from で運ぶ作り方 | 第12巻 UNIT 02、第13巻 UNIT 10 |
| `#[must_use]` | 戻り値を捨てたら警告する印。`Result` にも付いている | 第4巻 UNIT 05・10 |
| `mut` | 変数の中身を後から入れ替えられるようにする印 | 第2巻 UNIT 01 |
| Mutex | ミューテックス。中のデータを同時に1人だけがさわれるようにする鍵（標準と tokio の2種類） | 第5巻 UNIT 05 |
| `Mutex<T>` | 一度に1つのスレッドだけが中身に触れるようにする鍵付きの型 | 第3巻 UNIT 10 |

### N

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| N+1 問題 | 一覧で1回、その N 件それぞれで1回ずつ SQL を送り、DB と N+1 回往復してしまうこと | 第7巻 UNIT 09 |
| newtype | 既存の型を包んで別の意味の型にする方法 | 第2巻 UNIT 07、第4巻 UNIT 01 |
| NFKC | Unicode の正規化の1つ。全角の英数字を半角にそろえる | 第8巻 UNIT 03、第8巻 UNIT 08 |
| NLL | 借用が「最後に使われた場所」で終わる仕組み（Non-Lexical Lifetimes） | 第3巻 UNIT 07 |
| `no such table` | SQLite の「その表はありません」というエラー | 第9巻 UNIT 07 |
| `NodeViewParts` | 自分の型を、`view!` の `( )` に入れられるようにする trait | 第6巻 UNIT 05 |
| `nonce` | ID トークンの使い回しを断るために、ID トークンに入れてもらう値 | 第8巻 UNIT 09 |
| `not_found!()` | どこにも当たらない URL を `NotFoundError` にして、レイアウトまで届けるマクロ | 第6巻 UNIT 10 |
| `not_found()` | Topcoat の 404 のエラーを作る関数。`#[route]` の中で 404 を返すときに使う | 第9巻 UNIT 08 |
| `NoteFormState` | フォームに出す値とエラー。`from_input` で入力を残し、`with_errors` でエラーの文を入れる | 第10巻 UNIT 02 |
| `NoteInputErrors` | 作成・編集の検査で見つかった問題を、項目ごと（`title`・`body`）に入れる型 | 第10巻 UNIT 02 |
| `NoteQuery` | URL の `?q=`（キーワード）と `?page=`（ページ番号）を検査して入れる型 | 第10巻 UNIT 01、第10巻 UNIT 07 |
| NUL | 番号が 0 の文字。SQLite の `length()` は NUL より前だけを数える | 第11巻 UNIT 04 |
| NULL | ヌル。「値が無い」を表す特別な印。0 や `''` とは別物で、`IS NULL` で比べる | 第7巻 UNIT 01・02 |

### O

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| OFFSET | 並べた結果の先頭から、何行を飛ばすかの SQL の指定 | 第10巻 UNIT 08 |
| oha | Rust で作られた、HTTP の負荷をかける道具 | 第11巻 UNIT 09 |
| OIDC（OpenID Connect） | 外の認証サービスで本人を確かめ、結果を ID トークンで受け取る決まり | 第8巻 UNIT 09 |
| ON DELETE CASCADE | 親の行を消すと、それを指す子の行も一緒に消す指定 | 第7巻 UNIT 03、第9巻 UNIT 03、第10巻 UNIT 06、第11巻 UNIT 02 |
| OneDrive の同期フォルダ | 中のファイルをインターネット上に送り続けるフォルダ。作業フォルダには使わない | 第1巻 UNIT 01 |
| oneshot | 値を1回だけ届ける channel。依頼への返事に使う | 第5巻 UNIT 04 |
| `Option` | `Some(値)` か `None` | 第2巻 UNIT 08 |
| `ORDER BY … DESC` | 大きい（新しい）順に並べる指定。同じ値の行の順番は、決まりでは決まっていない | 第10巻 UNIT 01 |
| `Origin` | ブラウザが付ける、送り元のページの「方式・名前・ポート」。最後に / は付かない | 第8巻 UNIT 06 |
| `Origin` / `Sec-Fetch-Site` | ブラウザが付けるヘッダー。どのサイトのページから送ったかを表す | 第6巻 UNIT 10、第8巻 UNIT 01 |
| OriginPolicy | ほかのサイトのページから送られた、状態を変える要求を 403 で止める Topcoat の仕組み | 第6巻 UNIT 09、第8巻 UNIT 06、第10巻 UNIT 06、第13巻 UNIT 07 |
| OS | コンピュータの資源をプロセスに割り振る基本のソフト（Windows など） | 第0巻 UNIT 01 |
| os error 10048 | Windows の「アドレス（ポート）はもう使われています」 | 第9巻 UNIT 06、第9巻 UNIT 10、第12巻 UNIT 01 |
| os error 10061 | Windows の「接続を拒否されました」。相手のポートで、何も待ち受けていない | 第9巻 UNIT 10 |
| `overflow-checks` | `[profile.release]` に書くと、release でもあふれを確かめる設定 | 第12巻 UNIT 01 |
| `overflow-wrap: anywhere` | 空白の無い長い文字の並びでも、どこででも折り返してよい、という CSS の指定 | 第10巻 UNIT 10、第13巻 UNIT 09 |
| `owner: UserId` | リポジトリの全部の関数が受け取る持ち主。渡し忘れは E0061 | 第8巻 UNIT 07 |

### P

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| package | `Cargo.toml` 1つで管理される単位。lib と bin の crate を持てる | 第4巻 UNIT 06 |
| `#[page("/")]` | この関数は `GET /`（と HEAD）に答える、という Topcoat の属性マクロ | 第6巻 UNIT 02 |
| panic | 続行できない異常で、プログラムを止めること | 第2巻 UNIT 02、第3巻 UNIT 04・09 |
| `path_param::<Id>` | URL の変数を読む関数。`module_param!` が作った型（`Id`）で読む | 第13巻 UNIT 08 |
| `path_param::<Id>(cx)` | パスパラメーターを読む関数。型を付けたときは `Result` を返す | 第6巻 UNIT 04 |
| Pending / Ready | poll の答え。「まだです」と「できました（値つき）」 | 第5巻 UNIT 01 |
| `PER_PAGE` | 1ページの件数。RustNote は 20 | 第10巻 UNIT 08 |
| permit | 許可証。Semaphore からもらい、片付けられるときに返る | 第5巻 UNIT 05 |
| PHC 文字列 | `$argon2id$v=19$m=…$塩$ハッシュ` の形で、方式・設定・塩・ハッシュをまとめた保存用の文字列 | 第5巻 UNIT 07、第8巻 UNIT 02、第13巻 UNIT 06 |
| pin | ピン。Future をメモリの中で動かないように留めること | 第5巻 UNIT 01 |
| `pin!` | Future を動かないように留めるマクロ。留めると `&mut` で借りて何度も poll できる | 第5巻 UNIT 01・03 |
| PITR | point-in-time recovery。Turso Cloud が COMMIT ごとに残す backup から、過去の時刻の状態で新しい DB を作るしくみ | 第7巻 UNIT 10 |
| PKCE | code_verifier のハッシュ（code_challenge）を先に渡し、交換のときに code_verifier を見せる仕組み | 第8巻 UNIT 09 |
| `placeholder` | 入力欄の中の、薄い見本の文字。入力すると消えるので、ラベルの代わりにならない | 第9巻 UNIT 09 |
| Playwright | ブラウザをプログラムから動かす道具。`channel: "chrome"` でパソコンの Chrome を使える | 第11巻 UNIT 05 |
| point-in-time recovery | Turso で、過去の時刻の状態の DB を、新しい DB として作ること | 第12巻 UNIT 10 |
| poll | ポール。Future に「進められるところまで進めて」と頼むこと | 第5巻 UNIT 01 |
| PowerShell | Windows のシェル。この教材の主な作業環境 | 第0巻 UNIT 03 |
| PowerShell 7 | 別にインストールする新しい PowerShell（コマンド名は `pwsh`）。文字コードの既定などが 5.1 と違う | 第1巻 UNIT 01 |
| `PRAGMA` | DB の設定を読んだり変えたりする命令（`foreign_keys`・`busy_timeout` など） | 第7巻 UNIT 01・05 |
| `PRAGMA foreign_keys` | 外部キーの確かめを ON にする SQLite の設定。SQLite では接続ごとに入れる | 第13巻 UNIT 02 |
| `PRAGMA integrity_check` | DB のファイルが壊れていないかを調べる命令。問題がなければ `ok` を返す | 第7巻 UNIT 10、第12巻 UNIT 10 |
| PRG | POST の後に 303 で GET のページへ移動させる作法 | 第0巻 UNIT 05、第5巻 UNIT 08、第6巻 UNIT 01、第9巻 UNIT 02、第10巻 UNIT 02、第13巻 UNIT 07 |
| primary | プライマリ。コピーの元になる本体の DB | 第7巻 UNIT 08 |
| primitive obsession | 何でも `i64` や `String` のまま扱ってしまう癖。取り違えや未検証の値の原因 | 第4巻 UNIT 01 |
| procedure | ブラウザから `POST` で呼ばれる、サーバーの async 関数。`#[procedure("/api/…")]` で URL を決められる | 第6巻 UNIT 09 |
| `--profile minimal` | rustup で、rustc・cargo・標準ライブラリだけを入れる組み合わせ | 第1巻 UNIT 06 |
| Prometheus | プロメテウス。メトリクスを集めてグラフにする、定番のソフト | 第5巻 UNIT 09 |
| property test | どんな入力でも成り立つ決まりを、道具が作るたくさんの入力で確かめるテスト | 第11巻 UNIT 04 |
| proptest | Rust の property test のクレート。`proptest!` と strategy で入力を作る | 第11巻 UNIT 04 |
| `.proptest-regressions` | 失敗した入力の種を保存するファイル。Git に入れて共有する | 第11巻 UNIT 04 |
| `pub` | モジュールの外に公開する印 | 第2巻 UNIT 10 |
| `pub mod` | ファイルを、クレートの部品（モジュール）にする宣言 | 第13巻 UNIT 03 |
| `pub use` | 中の物を、別の道のりでも公開する。ファイルの並びと、外に見せる道のりを分けられる | 第9巻 UNIT 04 |
| `pub(crate)` / `pub(super)` | crate の中まで／親の module まで、に公開範囲を限る印 | 第4巻 UNIT 06 |
| `public_message` | 画面に出してよいエラーの文。500 のときは決まり文句にする | 第10巻 UNIT 09、第13巻 UNIT 04 |

### Q

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `#[query_params]` | クエリを読み込む構造体に付ける印。`query_params::<T>(cx)` で読む | 第6巻 UNIT 04、第10巻 UNIT 07、第13巻 UNIT 08 |
| `&quot;` | HTML の属性の値の中で、`"` を、ただの文字として書く書き方 | 第10巻 UNIT 02 |

### R

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| RAII | 資源の確保を値の作成に、解放を値の片付けに結びつける設計 | 第3巻 UNIT 01 |
| `Rc<T>` | 1つのスレッドの中で、持ち主を複数にする参照カウント付きの型 | 第3巻 UNIT 09 |
| read-your-writes | 書き込みが成功したら、書いたアプリ自身のコピーにはすぐ反映されること | 第7巻 UNIT 08 |
| `#[record]` | 自分の構造体を、signal や procedure で使えるようにする印 | 第6巻 UNIT 09 |
| `RefCell<T>` | 借用のルールを実行時にチェックする型。1つのスレッド専用 | 第3巻 UNIT 09 |
| release command | 新しい版のマシンを入れ替える前に、新しいイメージで1回だけ動かすコマンド。マイグレーションをここで済ませる | 第12巻 UNIT 07 |
| release ビルド | 最適化を有効にしたビルド（`--release`）。debug ビルドより計算がずっと速い | 第5巻 UNIT 07、第11巻 UNIT 08、第12巻 UNIT 01 |
| release_command | Fly.io で、新しい版を公開する前に1回だけ動かすコマンド（第12巻） | 第5巻 UNIT 10 |
| `--remap-path-prefix` | 実行ファイルに入れるフォルダの場所の文字を、別の文字に置き換える rustc の指定 | 第12巻 UNIT 01 |
| `RemoteAddr` | 接続してきた相手のアドレス。Topcoat のサーバーがリクエストに付ける（テストでは自分で付ける） | 第8巻 UNIT 10 |
| `RepoError` | RustNote の DB の層の失敗（一意制約の違反・version の食い違い・見つからない など） | 第13巻 UNIT 04 |
| Repository | リポジトリ。データのしまい方・取り出し方を引き受ける部品 | 第4巻 UNIT 02・04、第7巻 UNIT 09 |
| request_id | リクエストごとの受付番号。同時のリクエストのログを見分ける | 第5巻 UNIT 09 |
| Requests/sec | 1秒あたりに返せたリクエストの数 | 第11巻 UNIT 09 |
| reqwest | リクエスト。Rust でよく使われる HTTP クライアントのクレート | 第5巻 UNIT 08 |
| restart policy | マシン（コンテナ）のプロセスが終わったとき、動かし直すかの決まり。Fly.io の既定は on-fail | 第12巻 UNIT 06 |
| `Result` | `Ok(値)` か `Err(理由)` | 第2巻 UNIT 09 |
| RETURNING | `INSERT`・`UPDATE`・`DELETE` の後に、変わった行を返してもらう指定 | 第7巻 UNIT 03、第10巻 UNIT 05、第13巻 UNIT 05 |
| `role="status"` | 読み上げソフトに「お知らせの文です」と伝える属性 | 第9巻 UNIT 09、第10巻 UNIT 09 |
| rollback | 新しい版に問題があったとき、前の版（イメージ）に戻すこと。DB の中身は戻らない | 第12巻 UNIT 10 |
| rolling | マシンを順番に入れ替える公開のやり方 | 第12巻 UNIT 07 |
| `#[route(GET)]` | 文字や JSON などを、そのまま応答にする関数の印。レイアウトに包まれない | 第6巻 UNIT 03 |
| `Router::handle` | サーバーを起動せずに、ルーターに要求を1つ処理させるメソッド。テストで使う | 第6巻 UNIT 07、第9巻 UNIT 08、第13巻 UNIT 10 |
| `..row(…)` | 構造体を作るとき、残りの項目をほかの値から写す書き方 | 第9巻 UNIT 08 |
| runtime | Topcoat がページに入れる、ブラウザの中で動くプログラム。`$( … )` の式をブラウザでも動かす（0.11 でも実験的） | 第6巻 UNIT 09 |
| rust-analyzer | ラスト・アナライザー。Rust の補完・型の表示・チェックをする拡張機能 | 第1巻 UNIT 03 |
| `rust-toolchain.toml` | そのフォルダの中で使う Rust の版を決めるファイル | 第1巻 UNIT 01・03、第13巻 UNIT 01 |
| `rust-version` | その package に必要な Rust の最小の版（本書は `"1.99"`） | 第4巻 UNIT 07、第13巻 UNIT 01 |
| `RUST_LOG` | ログの細かさを決める環境変数（`with_env_filter` で読む） | 第5巻 UNIT 09、第11巻 UNIT 07 |
| rustc | Rust のコンパイラ。ソースコードを機械語にする | 第1巻 UNIT 04 |
| rustdoc | `///`・`//!` のコメントからドキュメントを作る仕組み（`cargo doc`） | 第4巻 UNIT 10 |
| `RUSTFLAGS` | cargo が rustc に渡す追加の指定を書く環境変数。変えると全部のクレートが作り直しになる | 第12巻 UNIT 01 |
| RustSec | Rust のクレートの弱点や、手入れが止まった知らせを集めたデータベース | 第11巻 UNIT 10 |
| rustup | ラストアップ。Rust の版（ツールチェーン）を入れたり選んだりする道具 | 第1巻 UNIT 04 |
| RwLock | 読み書き鍵。読む人は何人でも同時に、書く人は1人だけ | 第5巻 UNIT 05 |
| `RwLock<T>` | 読む人は同時に何人でも、書く人は1人だけ、を実行時に守る鍵 | 第3巻 UNIT 10 |

### S

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| salt（ソルト） | ハッシュにする前にパスワードに混ぜる、毎回ランダムな値。ハッシュと一緒に保存する | 第8巻 UNIT 02 |
| same-site | 方式と登録した名前（example.com の部分）が同じこと。別の名前（サブドメイン）でも same-site | 第8巻 UNIT 06 |
| `SameSite=Lax` | ほかのサイトからの POST などには付けず、リンクでページを開くときには付けるクッキーの属性 | 第6巻 UNIT 10、第8巻 UNIT 06、第10巻 UNIT 06 |
| SCAN / SEARCH | 全部を見る／索引で目的の場所へ直接飛ぶ | 第7巻 UNIT 04 |
| schema_migrations | 適用済みの migration の版を記録する表 | 第7巻 UNIT 06、第9巻 UNIT 07、第13巻 UNIT 02 |
| `scraper` | HTML を読み、CSS セレクターで要素を探すクレート | 第11巻 UNIT 03 |
| scrypt | メモリを多く使う、パスワード用のハッシュの方法。Better Auth の既定 | 第8巻 UNIT 08 |
| `Sec-Fetch-Site` | ブラウザが付ける「どこから来たか」の印。same-origin・same-site・cross-site・none | 第8巻 UNIT 06 |
| `Secret` | 秘密の値を包む型。`{:?}` では `Secret(********)` と表示し、`Display` は無い | 第9巻 UNIT 06、第13巻 UNIT 03 |
| secret | パスワードやトークンのような秘密の設定の値。Fly.io では fly secrets set で保管し、起動のときに環境変数で渡す | 第12巻 UNIT 05 |
| `Secret` 型 | 秘密の値を包み、`{:?}` で表示しても中身を `********` で隠す型 | 第7巻 UNIT 08 |
| `see_other` | 303 See Other の返事を作る関数。ページでは `Err(… .into())` で返す | 第6巻 UNIT 06 |
| seed | 開発や画面の確かめのために、最初から入れておく見本のデータ | 第9巻 UNIT 07 |
| Semaphore | セマフォ。決まった枚数の許可証で、同時に入れる数を決める道具 | 第5巻 UNIT 05、第11巻 UNIT 09 |
| SemVer | セムバー。版を「大.中.小」で表し、互換性の約束を決める方式 | 第4巻 UNIT 07 |
| Send | 別のスレッドへ渡してよい型の印 | 第3巻 UNIT 10、第5巻 UNIT 02 |
| `Send` / `Sync` | スレッドをまたいで送ってよい／同時に共有してよい、を表す trait | 第4巻 UNIT 04、第6巻 UNIT 07 |
| Serde | サーデ。値と JSON などの形式を相互に変換する仕組み（serde 1.0.229） | 第4巻 UNIT 08 |
| serde / `Deserialize` | データの形の変換を自動で書く道具。`Deserialize` は JSON などから struct を作る | 第5巻 UNIT 08 |
| `session::refresh` | 同じトークンのまま、期限を延ばす関数。RustNote は使っていない | 第8巻 UNIT 05 |
| `session::start` | 新しいトークンを作ってクッキーに入れ、`token_hash` と `expires_at` を返す Topcoat の関数 | 第8巻 UNIT 05 |
| `session::stop` | クッキーを消すように伝え、そのトークンの `token_hash` を返す関数 | 第8巻 UNIT 05 |
| `Set-Cookie` | クッキーを渡すヘッダー。`HttpOnly`・`Secure`・`SameSite`・`Path`・`Max-Age` などの属性が付く | 第11巻 UNIT 03 |
| `settings.json` | VS Code の設定のファイル。ユーザーの設定とワークスペースの設定がある | 第1巻 UNIT 03 |
| SHA-256 の指紋 | ファイルの中身から計算した長い値。1 バイトでも違えば、まったく別の値になる。`Get-FileHash` で表示できる | 第12巻 UNIT 01 |
| shard | 引数や signal が変わると、サーバーで描き直される部品。自分の入口（URL）を持つ | 第6巻 UNIT 09 |
| Shift_JIS | 日本語の Windows の古い文字コード。PowerShell 5.1 は BOM の無いスクリプトをこれで読む | 第1巻 UNIT 01 |
| shrink | 失敗した入力を、失敗したまま、小さく縮めること | 第11巻 UNIT 04 |
| signal | ブラウザが持つ状態。値が変わると、使っている式がブラウザで計算し直される | 第6巻 UNIT 09 |
| SIGTERM | Linux などで、OS やサービスがプログラムに「止まってください」と伝える合図 | 第5巻 UNIT 06 |
| SLO | 成功の割合・速さなど、利用者から見た良さの目標を、数で決めたもの | 第11巻 UNIT 07 |
| `Slot` | レイアウトが受け取る、内側のページ（または内側のレイアウト）の中身 | 第6巻 UNIT 03 |
| `#[source]` | thiserror の印。元のエラーを「原因」として持っておき、ログに原因まで書けるようにする | 第13巻 UNIT 04 |
| `spawn_blocking` | 同期のクロージャを blocking スレッドで動かし、引換券を返す関数。始まったら取り消せない | 第5巻 UNIT 07、第8巻 UNIT 02、第13巻 UNIT 06 |
| SQL | データベースにお願いするための言葉 | 第0巻 UNIT 07、第7巻 UNIT 01 |
| SQL インジェクション | 入力が SQL の文法として解釈され、意図と違う SQL が実行される脆弱性 | 第7巻 UNIT 07、第10巻 UNIT 07 |
| sqld | libSQL のサーバーのプログラム。HTTP で SQL を受け取る | 第1巻 UNIT 09、第12巻 UNIT 06 |
| SQLite | エスキューライト。1つのファイルに表を保存する、小さな DB の仕組み | 第7巻 巻頭 |
| SQLITE_CANTOPEN（14） | SQLite がファイルを開けないときの番号。無いフォルダを指定したときなど | 第9巻 UNIT 06、第13巻 UNIT 02 |
| `sqlite_sequence` | `AUTOINCREMENT` の次の番号を、SQLite が自動で記録する表 | 第13巻 UNIT 02 |
| `SqliteFailure` | libsql のエラーの種類の1つ。SQLite のエラー番号と文を持つ | 第7巻 UNIT 07 |
| stable | いちばん新しい安定版。`rustup update` で新しくなる | 第1巻 UNIT 04 |
| `state` | コールバックが、自分の始めたログインの続きかを確かめる値 | 第8巻 UNIT 09 |
| `'static` | プログラムの最後まで有効な期間。文字列リテラルや `static` | 第3巻 UNIT 06、第5巻 UNIT 02 |
| `static` | プログラム全体で1つだけ存在する値を作る宣言 | 第3巻 UNIT 06 |
| `std::error::Error` | エラーを表す型の標準の trait。`source()` で原因をたどれる | 第4巻 UNIT 05 |
| `&str` | 文字列を借りて読むための型 | 第2巻 UNIT 05 |
| strategy | 入力の作り方。正規表現の文字列・`any::<u32>()`・`prop_oneof!` など | 第11巻 UNIT 04 |
| STRICT | 列の型と違う値を断るようにする、表の指定 | 第7巻 UNIT 01 |
| strict mode | 探した要素が1つに決まらないときに止まる、Playwright の決まり | 第11巻 UNIT 05 |
| `String` | 書き足せる文字列の持ち主 | 第2巻 UNIT 05 |
| `sub` | 認証サービスの中で変わらない、利用者の番号 | 第8巻 UNIT 09 |
| subscriber | サブスクライバー。ログを受け取り、絞り込み・形の決定・書き出しをする係。プログラムに1つ | 第5巻 UNIT 09、第11巻 UNIT 07 |
| Sync | 複数のスレッドから同時に `&T` で読んでよい型の印 | 第3巻 UNIT 10 |
| sync / sync_interval | 手元のコピーを最新にすること／それを裏で自動的に行う間隔 | 第7巻 UNIT 08 |

### T

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| target | 作る実行ファイルが、どの CPU・OS 用か。`aarch64-pc-windows-msvc` など | 第1巻 UNIT 05 |
| `target` フォルダ | cargo がビルドした物を置くフォルダ。Git には入れない | 第1巻 UNIT 04 |
| TaskTracker | 作ったタスクを覚えておき、`close` の後の `wait` で全部の終わりを待つ係 | 第5巻 UNIT 06 |
| TCP | 2台の間で、データを順番どおり確実に届けるしくみ。HTTP はこの上を流れる | 第0巻 UNIT 05 |
| `TcpListener` | 決まったアドレスとポートで、接続を待つ標準ライブラリの型 | 第6巻 UNIT 01 |
| `TempDir` | `tempfile` クレートの一時フォルダ。片付けられる（drop される）と、フォルダごと消える | 第11巻 UNIT 02 |
| `tempfile::tempdir()` | 一時フォルダを作る。返す `TempDir` が捨てられると、フォルダごと消える | 第13巻 UNIT 10 |
| `text/plain` | 本文が、HTML ではない、ただの文字であることを表す種類 | 第9巻 UNIT 08 |
| thiserror | エラー型の `Display` と `Error` を derive で作るクレート（2.0.21） | 第4巻 UNIT 05、第13巻 UNIT 04 |
| `thread::scope` | 中で作ったスレッドが必ず先に終わるので、外の変数を借りられる仕組み | 第3巻 UNIT 10 |
| tie-breaker | タイブレーカー。並べ替えで同点の行の順番を決める最後の列（主キー） | 第7巻 UNIT 02 |
| timeout | タイムアウト。待つ時間の上限。`tokio::time::timeout(上限, Future)` | 第5巻 UNIT 06 |
| `token_hash` | トークンの SHA-256。DB の `sessions` 表には、これだけを保存する | 第8巻 UNIT 01 |
| `TokenHash` | トークンの SHA-256。記録には、トークンではなくこれを保存する | 第6巻 UNIT 10、第13巻 UNIT 05 |
| Tokio | トーキョー。Future を動かすランタイムを提供するクレート | 第5巻 巻頭・UNIT 02 |
| tokio-util | tokio と一緒に使う道具の集まり。`CancellationToken`・`TaskTracker` など | 第5巻 UNIT 06 |
| `tokio::join!` | 複数の Future を同時に進め、全部の結果をタプルで返す | 第5巻 UNIT 01、第10巻 UNIT 05 |
| `tokio::select!` | 複数の Future を同時に進め、最初に終わった枝だけを実行するマクロ | 第5巻 UNIT 03 |
| `tokio::signal::ctrl_c` | Ctrl+C が押されるまで待つ Future。待ち受け中は強制終了されない | 第5巻 UNIT 06 |
| `tokio::spawn` | Future をタスクとして動かし始め、引換券を返す関数 | 第5巻 UNIT 02 |
| `#[tokio::test]` | async のテスト関数に付ける印。テストの外側に Tokio のランタイムを用意する | 第9巻 UNIT 01、第11巻 UNIT 01 |
| `tokio::try_join!` | `Result` を返す Future 用の `join!`。最初の失敗をすぐに返す | 第5巻 UNIT 01 |
| `tomllib` | Python の標準の、TOML を読むための部品 | 第12巻 UNIT 04 |
| Topcoat | Rust で Web アプリを作るライブラリ。このシリーズは 0.11.0 | 第1巻 UNIT 08 |
| `topcoat asset clean` | 作った束を消すコマンド | 第6巻 UNIT 08 |
| `topcoat asset list` | 束に入る予定のファイルを表示するコマンド | 第6巻 UNIT 08 |
| `topcoat dev` | ビルド・束・起動をまとめてして、保存のたびにやり直す開発用のサーバー | 第1巻 UNIT 08、第6巻 UNIT 08 |
| topcoat-cli | Topcoat の CLI（`topcoat` コマンド）のパッケージ名 | 第1巻 UNIT 08 |
| `topcoat::serve` | 自分で押さえた待ち受けの場所（`TcpListener` など）とルーターを受け取って、要求をさばく | 第6巻 UNIT 02 |
| `topcoat::start` | 環境変数 `HOST`・`PORT`（無ければ 127.0.0.1:3000）で待ち受けを始める。起動しても何も表示しない | 第6巻 UNIT 02 |
| TOTP | 秘密の鍵と時刻から、30 秒ごとに変わる数字を作る仕組み（RFC 6238） | 第8巻 UNIT 09 |
| toxiproxy | 2つのプログラムの間にはさんで、通信をわざと遅くしたり切ったりできる道具 | 第12巻 UNIT 09 |
| tracing | トレーシング。ログやスパンを出す側のクレート | 第5巻 UNIT 09、第11巻 UNIT 07 |
| `#[track_caller]` | 関数の中で失敗したとき、その関数を呼んだ行を、失敗の場所として表示させる印 | 第11巻 UNIT 03 |
| trait | 型が守る約束（メソッドの一覧）。第4巻で詳しく学ぶ | 第3巻 UNIT 08、第4巻 UNIT 02 |
| trait object `dyn Trait` | ある trait を守る「何かの型」。実行時に型が決まる | 第3巻 UNIT 08 |
| trait オブジェクト | `dyn Trait`。その trait を実装した何かの型の値。型は実行時まで分からない | 第4巻 UNIT 04 |
| trait 境界 | trait bound。型パラメータに「この trait を実装していること」を求める条件 | 第4巻 UNIT 02・03 |
| transaction | トランザクション。複数の SQL を「全部確定か、全部取り消しか」にまとめる仕組み | 第7巻 UNIT 05 |
| `trust_origins` | 決まった送り元からの、変えるリクエストを通す OriginPolicy の設定 | 第8巻 UNIT 06 |
| `try_app_context::<T>(cx)` | 登録した値を借りる関数。登録していなければ `None` | 第6巻 UNIT 07 |
| `try_send` | 待たずに送る。箱がいっぱいなら、すぐに `Full` のエラーを返す | 第5巻 UNIT 04 |
| Turso | libSQL の DB を貸すクラウドのサービス | 第1巻 UNIT 09、第7巻 巻頭 |
| Turso / libSQL | RustNote が使うデータベースのサービスと、その中身のデータベース | 第0巻 UNIT 07 |
| Turso CLI | Turso を操作するコマンド（`turso`）。Windows 版は無く、WSL の中で使う | 第1巻 UNIT 09 |
| `turso db tokens invalidate` | その DB（同じグループ）のトークンを全部使えなくする Turso のコマンド。短い停止がある | 第12巻 UNIT 05 |

### U

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| UNIQUE 制約 | 同じ値の行を2つ作れないという、表の列の決まり | 第8巻 UNIT 03 |
| `UniqueViolation` | UNIQUE に止められたことを表す RustNote の `RepoError`。登録では 409 になる | 第8巻 UNIT 03 |
| `Unpin` | poll の途中で場所を動かしても平気な型に付く印。`Sleep` などは `Unpin` ではない | 第5巻 UNIT 03 |
| `unrecognized token` | SQLite の「その記号（言葉）は知りません」というエラー。SQL の書き間違い | 第10巻 UNIT 05 |
| URL | Web 上の宛先。スキーム・ホスト・ポート・パス・クエリでできている | 第0巻 UNIT 05 |
| `use` | 長いパスを短い名前で使えるようにする宣言 | 第2巻 UNIT 10 |
| `User-Agent` | 要求を送ったプログラム（ブラウザや curl）の名前を入れるヘッダー | 第6巻 UNIT 04 |
| `UserRecord` | パスワードのハッシュ付きのユーザー。ログインの照合のときだけ使う | 第13巻 UNIT 05 |
| `usize` | 長さ・個数・添え字に使う、PC に合わせた幅の整数 | 第2巻 UNIT 02, 06 |
| UTC | 世界の基準の時刻。日本時間は UTC より 9 時間進んでいる | 第13巻 UNIT 08 |
| UTF-16 の単位 | 文字を 16 ビットずつで表したときの数。ブラウザの `maxlength` はこれで数える | 第11巻 UNIT 01 |
| UTF-8 | 世界中の文字を表せる文字コード。Rust の文字列はこれ | 第0巻 UNIT 03、第2巻 UNIT 05、第3巻 UNIT 04 |

### V

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `VACUUM INTO` | DB を開いたまま、つじつまの合った写しを別のファイルに書き出す SQL。同じ名前のファイルがあると失敗する | 第7巻 UNIT 10、第12巻 UNIT 10 |
| `Value` | libsql で、種類の分からない値をそのまま受け取る enum | 第7巻 UNIT 07 |
| `vary` | この応答が、要求のどのヘッダーによって変わるかを伝えるヘッダー | 第6巻 UNIT 02、第12巻 UNIT 03 |
| `Vec` | 増減できる並び | 第2巻 UNIT 06 |
| `version` | ノートの版の番号。保存のたびに 1 増える。編集のフォームの hidden で運ぶ | 第10巻 UNIT 04 |
| `VersionMismatch` | 保存の SQL が 0 行で、ノートはあるとき（版が違う）のリポジトリのエラー。409 になる | 第10巻 UNIT 04、第11巻 UNIT 02 |
| `view!` | HTML をほぼそのまま書いて、ページの中身を作るマクロ | 第6巻 UNIT 02、第6巻 UNIT 05 |
| `viewport` | スマートフォンで、画面の幅に合わせて表示するための `<meta>` の設定 | 第9巻 UNIT 09 |
| `viewport` の meta | スマートフォンで、画面の幅に合わせて表示するための指定 | 第10巻 UNIT 10 |
| volume | コンテナの外に置く、データの置き場。コンテナを作り直しても残る | 第13巻 UNIT 10 |
| VS Code | Visual Studio Code。この本で使うコードエディタ | 第1巻 UNIT 03 |
| vswhere.exe | 入っている Visual Studio・Build Tools を答える道具 | 第1巻 UNIT 05 |
| vtable | trait object が持つ、その型用のメソッドの一覧（操作表） | 第3巻 UNIT 08、第4巻 UNIT 04 |

### W

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| Waker | ウェイカー。準備ができた Future を、もう一度 poll するよう知らせる起こし係 | 第5巻 UNIT 01 |
| WAL | Write-Ahead Logging。書き込み中でも読み込みを待たせにくくする記録方式 | 第7巻 UNIT 05、第12巻 UNIT 10、第13巻 UNIT 02 |
| `-wal`・`-shm` | WAL の書き込みの記録と、その目次のファイル。DB の一部なので、本体と組で扱う | 第12巻 UNIT 10 |
| watch | 最新の値だけを共有する channel。設定の変更や「止めて」の合図に使う | 第5巻 UNIT 04 |
| WCAG | Web のアクセシビリティの国際的な決まり。A・AA・AAA の段階がある | 第9巻 UNIT 09 |
| `Weak<T>` | 持ち主の人数に数えない弱い参照。循環参照を防ぐ | 第3巻 UNIT 09 |
| Web フレームワーク | 要求を読み、URL ごとの処理に振り分け、応答を返す、といった決まった仕事を引き受けてくれる道具 | 第6巻 UNIT 02 |
| `webServer` | Playwright の設定。テストの前にサーバーを起動し、URL が答えるまで待つ | 第11巻 UNIT 05 |
| `where` 句 | trait 境界を関数名の後ろにまとめて書く書き方 | 第4巻 UNIT 03 |
| `white-space: pre-wrap` | 改行や空白を、書いたとおりに表示する CSS の指定 | 第10巻 UNIT 03、第13巻 UNIT 09 |
| Windows PowerShell 5.1 | Windows に最初から入っている PowerShell。この本で使う版 | 第1巻 UNIT 01 |
| Windows Terminal | シェルの窓を開くアプリ。スタートメニューで「ターミナル」と探す | 第1巻 UNIT 01 |
| winget | ウィンゲット。Windows に入っている、コマンドでアプリを入れる道具 | 第1巻 UNIT 02 |
| worker | ワーカー。仕事の列から仕事を取り出して処理する係 | 第5巻 UNIT 10 |
| workspace | 複数の package をまとめ、`Cargo.lock` と `target` を共有する仕組み | 第4巻 UNIT 06 |
| `\wsl.localhost` | Windows から見た、WSL の Linux のフォルダの入口 | 第1巻 UNIT 07 |
| WSL2 | Windows の中で、本物の Linux を動かすしくみ | 第1巻 UNIT 07 |

### X

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| `X-Forwarded-For` | 中継のサーバーが、元の送り元の IP を伝えるためのヘッダー | 第8巻 UNIT 08 |
| x64 / AMD64 | Intel や AMD の CPU の体系。Windows では `AMD64` と表示される | 第1巻 UNIT 01 |
| XSS | 利用者の文字に混ぜたプログラムを、ほかの人の画面で動かす攻撃 | 第6巻 UNIT 05、第10巻 UNIT 02 |

### あ行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| アカウントの列挙 | 画面の返事の違いから、ある人が登録しているかどうかを調べること | 第8巻 UNIT 03、第8巻 UNIT 04 |
| 赤と緑 | テストが失敗している状態（赤）と、通っている状態（緑） | 第9巻 UNIT 08 |
| アクセシビリティ | 人によって目・耳・手の使い方が違っても、誰でも使えるようにすること | 第9巻 UNIT 09、第11巻 UNIT 05 |
| アクター | 自分だけのデータを持ち、メッセージで届いた依頼を1つずつ処理するタスク | 第5巻 UNIT 04 |
| アセット | CSS・画像・フォントなど、プログラムと一緒に配る、中身が決まったファイル | 第6巻 UNIT 08 |
| アセットの束 | CSS などを、指紋付きの名前でまとめたもの。`target\debug\assets` | 第1巻 UNIT 08、第12巻 UNIT 01、第12巻 UNIT 03 |
| アダプタ／消費メソッド | `map` などの途中の工程／`collect` などの最後の工程 | 第2巻 UNIT 10 |
| 圧縮 | 同じ情報のまま、送る中身を小さくすること。HTTP では gzip と br（Brotli）がよく使われる | 第12巻 UNIT 03 |
| アプリ（プログラム） | 決まった手順を書いたもの。動かしていない状態のもの | 第0巻 UNIT 01 |
| アプリのコンテキスト | アプリ全体で共有する値の置き場。`.app_context(値)` で登録し、型で見分ける | 第6巻 UNIT 07 |
| 暗号化 | 途中で盗み見や書き換えをされないように、データを変換すること。HTTPS で使う | 第0巻 UNIT 05、第8巻 UNIT 02 |
| 安全なメソッド | 何回送っても、サーバーの中のデータを変えないメソッド（GET・HEAD） | 第6巻 UNIT 01 |
| 依存 | あるモジュールが、ほかのモジュールやクレートを使っていること | 第9巻 UNIT 04 |
| 依存クレート | 自分のプログラムが使う、ほかの人が作ったクレート。そのクレートが使うクレートも、間接的な依存になる | 第13巻 UNIT 01 |
| 依存性の逆転 | 上の層が具体的な実装ではなく、間に置いた約束（trait）に頼る設計 | 第4巻 UNIT 02 |
| 依存の向き | どのモジュールが、どのモジュールを使ってよいかの決まり。上の段は下の段を使ってよく、下の段は上の段を使わない | 第9巻 UNIT 04 |
| 一意制約 | UNIQUE。DB の列に同じ値を2つ入れてはいけない決まり | 第4巻 UNIT 05 |
| 一括代入の脆弱性 | mass assignment。送られた項目をそのまま使い、`owner_id` などを書き換えられる弱点 | 第4巻 UNIT 08 |
| 一方向 | ハッシュ値から元のデータに戻せない性質 | 第0巻 UNIT 08 |
| イテレータ | 要素を1つずつ流す仕組み。消費されるまで動かない | 第2巻 UNIT 10 |
| イメージ | コンテナの設計図。Docker Hub などから取ってくる | 第1巻 UNIT 07 |
| 入れ替え（ローテーション） | 秘密の値を、決まった時期や漏れたおそれがあるときに、新しい値に替えること | 第12巻 UNIT 05 |
| インスタンス | 型から作った1つの値 | 第2巻 UNIT 07 |
| 受入条件 | ストーリーが「できた」と言える条件。「前提・操作・結果」の形で書く | 第9巻 UNIT 01 |
| 失われた更新（lost update） | あとから保存した内容が、先に保存した内容を、黙って上書きしてしまうこと | 第10巻 UNIT 04 |
| 薄い縦切り | 1つの機能を、画面から DB まで、すべての層を通して小さく作ること | 第9巻 UNIT 08 |
| 腕 | `match` の `パターン => 値` の1つ | 第2巻 UNIT 04 |
| 影響行数 | 書き換えの文が実際に変えた行の数。0 なら「対象が無かった」 | 第7巻 UNIT 03・07 |
| 永続化 | プログラムが終わってもデータが残るように保存すること | 第0巻 UNIT 07 |
| エイリアス | コマンドの別名。PowerShell の `ls` は `Get-ChildItem` の別名 | 第0巻 UNIT 03 |
| エスケープ | HTML で特別な意味を持つ文字を、ただの文字として表示される書き方に置き換えること | 第6巻 UNIT 05、第10巻 UNIT 01 |
| 枝 | `select!` の中の「パターン = Future => 式」の1つ1つ | 第5巻 UNIT 03 |
| エミュレーション | ARM64 の PC で、x64 用のプログラムを通訳しながら動かす仕組み | 第1巻 UNIT 01 |
| エラー（error） | 直さないとコンパイルが終わらない問題 | 第0巻 UNIT 09 |
| エラー伝播 | 失敗を呼び出し元へ渡していくこと | 第2巻 UNIT 09 |
| エラーの予算 | SLO の残り。99.9% なら 0.1% までは失敗してよい | 第11巻 UNIT 07 |
| 応答（response） | サーバーからの返事。1行目（版・状態コード・説明）・ヘッダー・空行・本文でできている | 第6巻 UNIT 01 |
| 送り元 | リクエストを送ってきた所（IP アドレスなど）。回数の制限の鍵に使う | 第8巻 UNIT 04 |
| 行の値の比べ | `(updated_at, id) < (?1, ?2)` のように、列を組にして順に比べる SQL の書き方 | 第10巻 UNIT 08 |
| 行の変換（row_to_note） | DB の1行を、検証済みのドメインの型に変える関数。失敗したら場所を添えたエラーにする | 第7巻 UNIT 09 |
| お知らせ（フラッシュ） | 移った先で一度だけ出すメッセージ。RustNote はクッキー `flash` で受け渡す | 第10巻 UNIT 02 |
| 表（テーブル） | 同じ形の行の集まり。`users`・`notes` など | 第7巻 UNIT 01 |
| 表駆動テスト | 入力と期待する結果を表にして、ループで確かめるテスト | 第4巻 UNIT 10 |
| 表の形のテスト（table-driven test） | 入力と期待する結果の組を表に並べ、同じ確かめ方で1行ずつ試すテスト | 第11巻 UNIT 01 |
| 表のテスト | 入口と利用者の組み合わせを表にして、全部を1回で確かめるテスト | 第8巻 UNIT 07 |
| オーバーフロー | 計算結果が型の範囲からはみ出すこと | 第2巻 UNIT 02 |

### か行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 回帰テスト | 一度作った守りや直した間違いが、あとの変更で元に戻っていないかを、毎回確かめるテスト | 第8巻 UNIT 10、第11巻 UNIT 06 |
| 改行コード | 行の終わりを表す見えない文字。CR（`\r`・`0D`）と LF（`\n`・`0A`） | 第1巻 UNIT 02 |
| 回数の制限（レート制限） | 決まった時間の中での失敗の回数に上限を決め、超えたら断ること | 第8巻 UNIT 04 |
| 開発者ツールの Console | ブラウザの中で JavaScript を動かして、ページを調べられる場所（F12 キー） | 第10巻 UNIT 10 |
| 拡張機能 | VS Code にあとから足す部品 | 第1巻 UNIT 03 |
| 拡張子 | ファイル名の最後の `.` から後ろ（`.rs` など）。中身の種類の目印 | 第0巻 UNIT 02 |
| 型 | 値の種類（整数・文字列・真偽値など） | 第2巻 UNIT 01 |
| 型推論 | 型を書かなくても、値や使われ方から型を決める仕組み | 第2巻 UNIT 01 |
| 型注釈 | `: u32` のように型を明示すること | 第2巻 UNIT 01 |
| 型の検査（`parse`） | 外から来た値を検査し、合格したときだけ専用の型にする関数 | 第9巻 UNIT 03 |
| カレントディレクトリ | 今いるフォルダ。相対パスの出発点 | 第0巻 UNIT 02 |
| 変わった行の数 | `execute` が返す数。`DELETE` で 0 なら、消せる行が無かった | 第10巻 UNIT 06 |
| 環境の診断 | 道具がそろっているか・版が合っているかを、決まった順番で確かめること | 第1巻 UNIT 10 |
| 環境変数 | OS やシェルが持つ「名前＝値」のメモ。設定や秘密を渡すのに使う | 第0巻 UNIT 03、第4巻 UNIT 09、第7巻 UNIT 08、第9巻 UNIT 06、第13巻 UNIT 03 |
| 監査の記録（監査ログ） | ログインの成功・失敗・ログアウトなど、守りに関わる出来事を、あとから調べられるように残す記録 | 第8巻 UNIT 10 |
| 監視 | アプリと DB が正しく動いているかを、ヘルスチェックやログで見続けること | 第7巻 UNIT 10 |
| 関数 | 引数を受け取り、処理をして戻り値を返すまとまり | 第2巻 UNIT 03 |
| 関連関数 | `Note::new` のように型から呼ぶ関数 | 第2巻 UNIT 07 |
| 外部キー（FOREIGN KEY） | ほかの表の主キーを指す列。指す先が実在しない値は入れられない | 第7巻 UNIT 01・03、第9巻 UNIT 03 |
| 画面遷移図 | 画面と画面のつながりを、矢印で描いた図 | 第9巻 UNIT 02 |
| 画面の状態 | 1つの画面が、条件によって変える表示（空・入力エラー・ページ送り・検索で0件など） | 第9巻 UNIT 02 |
| 機械語 | CPU がそのまま実行できる命令の並び | 第0巻 UNIT 04 |
| 既定の実装 | trait の中で中身まで書いたメソッド。実装側で上書きもできる | 第4巻 UNIT 02 |
| キャッシュ | 受け取ったファイルをブラウザが手元に取っておき、次は取りに行かずに使うこと | 第6巻 UNIT 08 |
| キャッシュのマウント | `RUN --mount=type=cache,…` で、ビルドをまたいでフォルダ（target/ など）を取っておくしくみ。イメージには残らない | 第12巻 UNIT 02 |
| キャンセル | Future が途中で drop され、二度と poll されなくなること | 第5巻 UNIT 01・02 |
| キャンセル安全 | 途中で捨てられても、データを失ったり中途半端な状態を残したりしない性質（cancel safety） | 第5巻 UNIT 03 |
| 脅威モデル | 守るもの・攻める人・入口・信頼の境目を書き出し、ありうる攻撃を整理したもの | 第8巻 UNIT 01 |
| 境界値 | 決まりが切り替わる境目の値。端と、その1つ外側を試す | 第11巻 UNIT 01 |
| 境界値テスト | 80 文字と 81 文字のように、境目の値を確かめるテスト | 第4巻 UNIT 10 |
| 供給網（サプライチェーン） | 自分のプログラムが使う部品と、その部品が使う部品のつながり | 第11巻 UNIT 10 |
| 競合状態（race condition） | 同時に進む処理の順番によって、結果が変わってしまうこと | 第10巻 UNIT 05 |
| 強制終了 | 処理中の仕事を待たずに、その場でプログラムを止めること | 第5巻 UNIT 06 |
| 共有しているもの | DB のファイル・ポート番号・環境変数・今の時刻など、テストどうしで同じものを使う所。ときどき失敗するテストの原因になる | 第11巻 UNIT 02 |
| 切り離されたタスク | 引換券を捨てて、誰も結果を待っていないタスク（detached task） | 第5巻 UNIT 02 |
| 行・列 | 表の1件分の記入欄（行、レコード）と、その項目（列、カラム） | 第7巻 UNIT 01 |
| クエリ | DB への問い合わせ。特に `SELECT` 文 | 第7巻 UNIT 02 |
| クエリ文字列 | URL の `?` より後ろの、`名前=中身` を `&` でつないだ部分 | 第6巻 UNIT 04 |
| クッキー | サーバーが `Set-Cookie` で預け、ブラウザが `Cookie` で送り返す、名前と値の組 | 第6巻 UNIT 10 |
| 組み込みレプリカ | embedded replica。Turso の DB のコピーを手元のファイルに置き、読むのはコピー・書くのは Turso にする形 | 第7巻 UNIT 08 |
| クライアント | サーバーに頼みごとをする側（ブラウザなど） | 第0巻 UNIT 06 |
| クレート | Rust のプログラムや部品のひとまとまり | 第0巻 UNIT 04、第2巻 UNIT 10、第9巻 UNIT 04 |
| クロージャ | その場で書く小さな関数。周りの変数を取り込める | 第2巻 UNIT 10 |
| 警告（warning） | コンパイルは通るが、間違いの可能性やより良い書き方を知らせる指摘 | 第0巻 UNIT 08 |
| 結合テスト | いくつかの部品を、本物どうしでつないで確かめるテスト。ここでは、リポジトリと本物の DB | 第11巻 UNIT 02 |
| 検査済みの型 | 決まりの検査に合格した値だけが入る型。中身を private にして、parse を通らないと作れないようにする | 第13巻 UNIT 04 |
| 原因の鎖（`error.chain()`） | エラーの原因を、外側から内側へ順にたどったもの。ログにだけ書く | 第10巻 UNIT 09 |
| 原因の連なり（source） | エラーを包んでいる奥のエラー。`error.source()` で1つずつたどれる | 第5巻 UNIT 08 |
| 構造化された並行処理 | 子のタスクが親より長生きしないように、親が子の始まりと終わりを管理する書き方 | 第5巻 UNIT 03 |
| 構造化ログ | 決まった欄に値を書くログ。検索や集計ができる | 第5巻 UNIT 09、第11巻 UNIT 07 |
| 構造体（struct） | 関係のある値をまとめた、自分で作る型 | 第0巻 UNIT 08、第2巻 UNIT 07 |
| 孤児ルール | orphan rule。trait か型の少なくとも一方が自分の crate のものでないと `impl` できない決まり | 第4巻 UNIT 02 |
| コマンド パレット | VS Code の操作を名前で探して実行する入力欄。`Ctrl + Shift + P` | 第1巻 UNIT 03 |
| コマンドライン引数 | コマンドの後ろに並べて、プログラムに渡す値 | 第0巻 UNIT 01 |
| コミット | ある時点のファイルの状態を、メッセージ・作った人・日時と一緒に記録したもの | 第1巻 UNIT 02 |
| コンテナ | アプリと、その動きに要る物をまとめて、切り分けて動かす箱 | 第1巻 UNIT 07 |
| コントラスト比 | 文字と背景の明るさの比。1〜21。AA はふつうの文字で 4.5 以上 | 第9巻 UNIT 09、第13巻 UNIT 09 |
| コンパイル | ソースコードを機械語に翻訳すること。翻訳する道具がコンパイラ（Rust では `rustc`） | 第0巻 UNIT 04 |
| コンポーネント | 何度も使える画面の部品。`#[component]` を付けた async 関数で、名前付きの引数で呼ぶ | 第6巻 UNIT 05 |

### さ行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 再帰型 | 自分と同じ型を中に含む型。`Box` などで間接化が必要 | 第3巻 UNIT 08 |
| 再現性 | 同じソース・同じ道具・同じ部品から、いつ誰が作っても同じ物ができること | 第12巻 UNIT 01 |
| 再公開 | `pub use`。別の場所の項目を、ここにもあるように見せること | 第4巻 UNIT 06 |
| 再試行（retry） | 失敗した操作を、もう一度行うこと | 第5巻 UNIT 08 |
| 再借用 | 可変参照から、さらに一時的に借りること。`&mut *r` | 第3巻 UNIT 07 |
| 最小権限 | プログラムに、要る分だけの権限しか与えない考え方。RustNote は app で動き、書けるのは /data だけ | 第12巻 UNIT 02 |
| 最適化 | 同じ動きのまま、速く小さくなるように、コンパイラーがプログラムを作り直すこと | 第12巻 UNIT 01 |
| 作業フォルダ | この本の練習をまとめて置くフォルダ。`~\rust-study` | 第1巻 UNIT 01 |
| 参照 | 値の場所を指す、持ち主ではない値。`&T` と `&mut T` がある | 第3巻 UNIT 03 |
| 参照カウント | 持ち主の人数を数え、0 になったら片付ける方式 | 第3巻 UNIT 09・10 |
| 参照外し `*` | 参照の先にある値そのものを取り出す操作 | 第3巻 UNIT 03 |
| 参照外し型強制 | `&String` → `&str` のように、参照の型を自動で変換する仕組み | 第3巻 UNIT 04 |
| サーバー | リクエストを待ち受けて、レスポンスを返す側のプログラム | 第0巻 UNIT 06 |
| サーバーの側のセッション | 記録を DB に持つ作り方。サーバーで消せば、すぐに使えなくできる | 第8巻 UNIT 05 |
| シェル | コマンドを読んで、OS にプログラムの起動などを頼むプログラム（PowerShell・bash） | 第0巻 UNIT 03、第1巻 巻頭・UNIT 01 |
| 塩（salt） | ハッシュにする前に足すランダムな値。同じパスワードでも、毎回違うハッシュになる | 第13巻 UNIT 06 |
| 式 | 評価すると値になるもの | 第2巻 UNIT 03 |
| シグネチャ | 関数の1行目。名前・引数の型・戻り値の型 | 第2巻 UNIT 03 |
| 指数バックオフ | 再試行の間隔を倍々にのばしていくこと。上限を付けて使う | 第5巻 UNIT 08 |
| 指紋（ハッシュ） | ファイルの中身から計算した値。中身が1文字でも変われば、別の値になる | 第12巻 UNIT 03 |
| 借用 | 参照を作ること。値を一時的に借りること | 第3巻 UNIT 03 |
| シャドーイング | 同じ名前で `let` を書き、新しい変数を作り直すこと | 第2巻 UNIT 01 |
| 集計関数 | `COUNT`・`SUM`・`MAX` など、たくさんの行を1つの値にまとめる関数 | 第7巻 UNIT 02 |
| 終了コード | プロセスが終わるときに OS へ渡す数。0 は成功 | 第0巻 UNIT 01、第1巻 UNIT 06、第9巻 UNIT 10 |
| 主キー | テーブルの1行を特定するための列（`id` など）。同じ値は2つない | 第0巻 UNIT 07、第7巻 UNIT 01 |
| 障害（インシデント） | アプリが使えない・データが消えた・秘密が漏れた、のように利用者に影響が出る出来事 | 第7巻 UNIT 10 |
| 省略規則 | ライフタイム注釈を自動で補う3つの規則 | 第3巻 UNIT 05 |
| 署名付きのクッキー | 値の横に署名を付けたクッキー。書き換えると署名が合わず、読まれない | 第6巻 UNIT 10、第8巻 UNIT 08 |
| 所有権 | 値を最後に片付ける責任を、どの変数が持っているか | 第3巻 全体 |
| シリアライズ / デシリアライズ | 値 → 文字列（JSON など）／文字列 → 値 の変換 | 第4巻 UNIT 08 |
| 診断（doctor） | 起動に要るものを1つずつ確かめて、どこが悪いかを表示する小さなプログラム | 第9巻 UNIT 10 |
| 信頼の境目 | ここより外から来た値は信じずに確かめ直す、という線 | 第8巻 UNIT 01 |
| 事故の対応（インシデント対応） | 気づく・止める・調べる・知らせる・直す・振り返るまでの、事故のときの動き | 第8巻 UNIT 10 |
| 実行ポリシー | どんなスクリプトなら実行してよいかを決める PowerShell の設定。この本では `RemoteSigned` | 第1巻 UNIT 01 |
| ジッター | 再試行の間隔に足す、ばらつき。やり直しの時刻を散らす | 第5巻 UNIT 08 |
| 循環（cyclic package dependency） | パッケージどうしが互いに依存している形。クレートは順番にコンパイルするので、Cargo が止める | 第9巻 UNIT 04 |
| 循環参照 | `Rc` どうしが指し合い、人数が 0 にならない状態 | 第3巻 UNIT 09 |
| 状態機械 | 「今の状態」と「操作」で「次の状態」が決まる仕組み | 第4巻 UNIT 10、第5巻 UNIT 01 |
| 状態コード | 応答の1行目の3桁の数。2xx 成功・3xx 別の場所へ・4xx 頼み方が違う・5xx サーバーの失敗 | 第6巻 UNIT 01 |
| 情報設計 | 画面に何を、どの順番で、どうまとめて置くかを決めること | 第9巻 UNIT 02 |
| ジョブキュー | 仕事の列。この巻では DB の `jobs` 表で作った | 第5巻 UNIT 10 |
| スキーマ | DB にどんな表・列・制約があるか、という形の定義 | 第7巻 UNIT 06、第9巻 UNIT 03 |
| 少なくとも1回（at-least-once） | 仕事が失われないかわりに、2回以上動くことがある、という約束 | 第5巻 UNIT 10 |
| スクリプト | コマンドをファイルに並べて書いたもの。PowerShell では `.ps1` | 第1巻 UNIT 01 |
| スケジューラー | 待ち行列のタスクを、ワーカースレッドに割り振る係 | 第5巻 UNIT 02 |
| スコープの外 | この版では作らないこと。「作り忘れ」と区別するために書いておく | 第9巻 UNIT 01 |
| スタックあふれ | 関数の作業用のメモリ（スタック）が足りなくなって、プログラムが異常終了すること | 第7巻 UNIT 08 |
| ステージ（インデックス） | 次のコミットに入れるものを並べておく台紙。`git add` で載せる | 第1巻 UNIT 02 |
| ステータスコード | HTTP のレスポンスの結果を表す3桁の数（200・404 など） | 第0巻 UNIT 05、第4巻 UNIT 05、第5巻 UNIT 08 |
| ストレージ | 電源を切ってもデータが残る記憶装置（SSD など） | 第0巻 UNIT 01 |
| ストレージクラス | SQLite の値の種類（integer・real・text・blob・null） | 第7巻 UNIT 01・07 |
| スパン | 処理の区間。中で出したログに情報を引き継ぎ、木の形にできる | 第5巻 UNIT 09 |
| スモークテスト | 公開した直後に、大事な使い方をひととおり、本物の URL で試すこと | 第12巻 UNIT 08 |
| 正規化 | 同じ意味のものを同じ形にそろえること（メールの小文字化など） | 第4巻 UNIT 01、第8巻 UNIT 03、第10巻 UNIT 07 |
| 整形（フォーマット） | コードの字下げや空白を、決まった形に直すこと。Rust では rustfmt | 第1巻 UNIT 03 |
| 制限モード | 信頼しなかったフォルダを開いたときの、拡張機能などを止めた状態 | 第1巻 UNIT 03 |
| 整数のあふれ（オーバーフロー） | 計算の答えが、型で表せる範囲を超えること。debug は panic、release は既定で折り返す | 第12巻 UNIT 01 |
| 静的ディスパッチ | 呼び出す関数をコンパイル時に決めること | 第4巻 UNIT 03・04 |
| 制約 | データベースの列に付けるルール（NOT NULL・UNIQUE・CHECK など） | 第0巻 UNIT 07、第7巻 UNIT 01 |
| セッション | ログインした状態を覚えておくしくみ。トークンを Cookie で受け渡す | 第0巻 UNIT 08、第8巻 UNIT 01、第8巻 UNIT 05 |
| セッションの固定 | 攻撃する人が知っているトークンを使わせてから、成りすますこと | 第8巻 UNIT 04 |
| セッションのトークン | ログインした人に渡す、32 バイトの乱数。`__Host-session` のクッキーで運ぶ | 第6巻 UNIT 10 |
| 絶対パス | ドライブ名（`C:\`）から書いた、どこからでも同じ場所を指すパス | 第0巻 UNIT 02 |
| ゼロ幅接合子（U+200D） | 絵文字どうしをつないで1つに見せる、見えない文字 | 第11巻 UNIT 01 |
| 前提・操作・結果（Given / When / Then） | 受入条件やテストを書くときの定番の形 | 第9巻 UNIT 01 |
| 総当たり | 考えられるパスワードを、片っ端から試す攻撃 | 第8巻 UNIT 02 |
| 早期リターン | 異常な場合を先に `return` で片付ける書き方 | 第2巻 UNIT 03 |
| 総合の試験（E2E） | いくつもの画面を、利用者が使う順番に1本の流れでたどるテスト | 第10巻 UNIT 10 |
| 相対輝度 | 色の明るさを 0（黒）〜1（白）で表した値。コントラスト比の計算に使う | 第9巻 UNIT 09 |
| 相対パス | 今いるフォルダから見た道順で書いたパス | 第0巻 UNIT 02 |
| 束（バンドル） | `topcoat asset bundle` が作る、ハッシュ付きの名前のファイルと目録のまとまり | 第6巻 UNIT 08 |
| 束縛 | 名前と値を結び付けること | 第2巻 UNIT 01 |
| 空の状態（empty state） | 表示するデータが1件も無いときの画面。次にすることを伝える文を出す | 第10巻 UNIT 01 |
| ソルト | パスワードのハッシュに混ぜる、人ごとに違う乱数 | 第0巻 UNIT 08 |
| ソースコード | 人が読み書きするプログラムの文章 | 第0巻 UNIT 04 |

### た行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 確かめの画面 | 「本当に削除しますか」と聞いてから、POST のボタンで消す画面。RustNote には無い | 第10巻 UNIT 06 |
| タスク | ランタイムの上で動く、1つの独立した仕事。`tokio::spawn` で作る | 第5巻 UNIT 02 |
| タプル | 型の違う値の、決まった数の組 | 第2巻 UNIT 06 |
| 単相化 | monomorphization。generics を使われた型ごとの専用の関数にコンパイルすること | 第4巻 UNIT 03 |
| 単体テスト / 結合テスト | 部品ごとのテスト／ライブラリを外から使う `tests/` のテスト | 第4巻 UNIT 10 |
| 端末（ターミナル） | 文字を入力して結果を表示する窓 | 第0巻 UNIT 03 |
| ターボフィッシュ | `parse::<u32>()` の `::<>` の書き方 | 第4巻 UNIT 03 |
| 代替データストリーム | NTFS で、1つのファイルの中に持てる別の流れ。`名前:流れ` と書く | 第1巻 UNIT 09 |
| ツールチェーン | ある版の Rust の道具一式。`stable-aarch64-pc-windows-msvc` のような名前 | 第1巻 UNIT 04 |
| 定数 | コンパイル時に決まり、変わらない値。型注釈が必須 | 第2巻 UNIT 01 |
| テスト | プログラムが期待どおりに動くかを確かめる小さなプログラム | 第0巻 UNIT 10 |
| テストの段 | ユニット・DB の結合・HTTP・E2E のように、確かめる範囲の広さで分けたテストの種類 | 第11巻 UNIT 01 |
| テストピラミッド | 速い単体テストを多く、遅いテストを少なくする考え方 | 第4巻 UNIT 10 |
| テーブル | データベースの表。行（1件）と列（項目）でできている | 第0巻 UNIT 07 |
| ディストリビューション | Linux の種類。この本では Ubuntu | 第1巻 UNIT 07 |
| データ競合 | 複数のスレッドが鍵なしで同じ場所に同時に読み書きすること | 第3巻 UNIT 10 |
| データベース（DB） | データを整理して保存し、探したり書き換えたりするプログラム | 第0巻 UNIT 07、第7巻 巻頭 |
| 統合ターミナル | VS Code の中で開くターミナル。開いたフォルダから始まる | 第1巻 UNIT 03 |
| トランザクション | いくつかの変更を、全部やるか全部やめるかの1まとまりにする仕組み。`commit` で確定、`rollback` で取り消す | 第13巻 UNIT 02 |
| 取り下げ（yank） | 問題のある版に、作った人が「新しく使わないで」と付ける印 | 第9巻 UNIT 05 |
| トレイト | 型に共通の機能を付ける仕組み（第4巻） | 第2巻 UNIT 09 |
| トークン | Turso の DB に入るための秘密の文字列。コードにもログにも出さない | 第7巻 UNIT 08、第8巻 UNIT 01 |
| トークンの無効化 | `turso db tokens invalidate`。署名の鍵を作り直し、その DB（と同じグループ）のトークンを全部使えなくする | 第7巻 UNIT 10 |
| 動的ディスパッチ | 実行時に、呼ぶメソッドを操作表から選ぶこと | 第3巻 UNIT 08、第4巻 UNIT 04 |
| 独自ドメイン | 自分のドメインで、アプリを開けるようにすること。DNS のレコードで Fly.io に向ける | 第12巻 UNIT 08 |
| ドメインモデル | アプリが扱う「もの」と、その性質とつながりを整理したもの | 第9巻 UNIT 03 |

### な行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 内部可変性 | `&T` しか持っていなくても中身を書き換えられる仕組み | 第3巻 UNIT 09 |
| 二重解放 | 同じメモリを2回返してしまう事故。Rust は move で防ぐ | 第3巻 UNIT 02 |
| 二重送信 | 同じフォームの POST が2回届くこと。PRG では防げない | 第10巻 UNIT 02 |
| 偽物（fake） | テストのために作った、本物の代わりの実装。SQL は動かないので、SQL の間違いには気づけない | 第11巻 UNIT 02 |
| 入力の検査（バリデーション） | 送られてきた値が決まりに合っているかを確かめること。合わなければ保存しない | 第10巻 UNIT 02 |
| 任意の依存 | `optional = true`。feature で ON にしたときだけ使う依存 | 第4巻 UNIT 07 |
| 認可 | その操作をしてよいかを判断すること | 第0巻 UNIT 08、第7巻 UNIT 03、第8巻 UNIT 01、第8巻 UNIT 07 |
| 認証 | 相手が誰かを確かめること（ログイン） | 第0巻 UNIT 08、第8巻 UNIT 01 |

### は行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 背圧 | 受け付けられる量を超えた分を、待たせたり断ったりすること | 第11巻 UNIT 09 |
| 配列 | 同じ型の値が決まった数だけ並んだもの | 第2巻 UNIT 06 |
| ハッシュ | データから決まった長さの値を作る一方向の変換。パスワードの保存に使う | 第0巻 UNIT 08、第8巻 UNIT 02 |
| 範囲 | 終わりを含まない／含む数の並び | 第2巻 UNIT 04 |
| 版の固定（pin） | 使う道具や部品の版を、1つに決めて書いておくこと | 第13巻 UNIT 01 |
| バイト | データの大きさの単位。UTF-8 では英数字が1バイト、日本語がふつう3バイト | 第0巻 UNIT 10 |
| バックグラウンドジョブ | 利用者への応答とは別に、裏で行う仕事 | 第5巻 UNIT 10 |
| バリアント | `enum` の候補の1つ | 第2巻 UNIT 08 |
| パス | ファイルやフォルダの場所を表す文字列。URL の中の `/notes` などもパスと呼ぶ | 第0巻 UNIT 02、第6巻 UNIT 01 |
| パスの引数 | URL の `{id}` のように、URL の一部を値として受け取るもの。`module_param!` で型を決める | 第10巻 UNIT 03 |
| パスパラメーター | URL のパスの中で変わる部分（`/notes/7` の `7`）。Topcoat では `{id}` と書く | 第6巻 UNIT 04 |
| パスワードスプレー | 1つのパスワードを、たくさんの人のメールで1回ずつ試す攻撃 | 第8巻 UNIT 04 |
| パターン | 値の形を合わせて中身に名前を付ける書き方 | 第2巻 UNIT 08 |
| パッケージ | `Cargo.toml` が1つのまとまり。中にクレートがいくつもある | 第9巻 UNIT 04 |
| パニック | Rust のプログラムが実行中に問題を見つけて止まること。終了コードは 101 | 第0巻 UNIT 01 |
| パラメーター（`?2`） | SQL に値を、文とは別に渡す仕組み。どんな文字が入っても、文として読まれない | 第10巻 UNIT 07 |
| パーセンタイル | 応答時間を短い順に並べたとき、何 % の所の値か（95%・99% など） | 第11巻 UNIT 09 |
| パーセントエンコーディング | URL に書けない文字を、`%` と2文字の 16 進数で1バイトずつ書く方法（`%E5%A4%AA` など） | 第6巻 UNIT 04 |
| ヒア文字列 | `@'` と `'@` ではさんだ、何行でもそのまま1つの文字列になる書き方 | 第1巻 UNIT 01 |
| 悲観的ロック | 直している間、ほかの人に触らせないように鍵をかける方法。Web の画面では使わない | 第10巻 UNIT 04 |
| 非機能要件 | 何ができるか以外の約束。安全さ・速さ・量の上限・動作の確かめ方など | 第9巻 UNIT 01 |
| ヒストグラム | 値を範囲ごとの箱に分けて数える数え方。応答時間の分布を見る | 第5巻 UNIT 09、第11巻 UNIT 07 |
| 秘密（シークレット） | パスワード・トークンなど、知られると悪用される値 | 第4巻 UNIT 09 |
| 標準エラー出力 | エラーの知らせを書く出口（`eprintln!`） | 第0巻 UNIT 01 |
| 標準出力 | ふつうの結果を書く出口（`println!`） | 第0巻 UNIT 01 |
| 標準ライブラリ | Rust に最初から付いてくる道具箱（`std`） | 第0巻 UNIT 01 |
| ピーク | 同時に借りていたメモリの量の、いちばん多い所 | 第11巻 UNIT 08 |
| フィールド | 構造体の中の1つ1つの値 | 第2巻 UNIT 07、第5巻 UNIT 09 |
| フォルダ（ディレクトリ） | ファイルやフォルダを入れる入れ物 | 第0巻 UNIT 02 |
| フォーカス | キーボードの入力を受け取る、画面の中の1つの場所。Tab で次へ、Shift+Tab で前へ | 第10巻 UNIT 10 |
| フォーカスの枠 | Tab キーで選んでいる所を示す枠（`outline`）。消さない | 第9巻 UNIT 09、第10巻 UNIT 10 |
| 負荷試験 | たくさんのリクエストを送り、速さと限界を測ること | 第11巻 UNIT 09 |
| 複合索引 | 複数の列を並べた索引。先頭の列から順にしか使えない | 第7巻 UNIT 04 |
| 符号なし／符号付き | マイナスを表せない／表せる整数の型 | 第2巻 UNIT 02 |
| 不正な状態を表現できない型 | 作れた時点で必ず正しい値になるよう、作り方を制限した型 | 第4巻 UNIT 01・10 |
| 伏せる（マスク） | 記録に残す値の一部を隠すこと（`alice@example.com` → `a…@example.com`） | 第8巻 UNIT 10 |
| 不変条件 | プログラムが動いている間、いつでも必ず成り立っていなければならない決まり | 第9巻 UNIT 03 |
| フラッシュメッセージ | 「保存しました」のように、次の画面で一度だけ出すお知らせ | 第9巻 UNIT 02、第10巻 UNIT 09、第13巻 UNIT 06 |
| フレームワーク | アプリでよく使う仕組みをまとめた部品集。Topcoat など | 第0巻 巻頭 |
| 物理削除 | DB の行を `DELETE` で本当に消すこと。元に戻せない | 第10巻 UNIT 06 |
| ブランチ | 変更の流れに付ける名前。このシリーズでは `main` | 第1巻 UNIT 02 |
| 文 | 値を持たない命令。`let ...;` など | 第2巻 UNIT 03 |
| 文脈（context） | 「何をしようとして失敗したか」をエラーに足した説明 | 第4巻 UNIT 05 |
| ブール属性 | `disabled`・`checked` のように、あるかないかだけで意味が決まる属性。値が `"false"` でも「ある」になる | 第6巻 UNIT 05 |
| プレースホルダー | SQL の中で、後から値を当てはめる場所取り（`?1`・`:owner`） | 第7巻 UNIT 02・07 |
| プロセス | 実行中のプログラム | 第0巻 UNIT 01 |
| プロトコル | やりとりの書き方の決まり。HTTP など | 第0巻 UNIT 05 |
| プロファイル | 時間やメモリが、どこで使われているかを調べること | 第11巻 UNIT 08 |
| プロンプト | コマンドの入力を待っている印（`PS C:\Users\you>`） | 第0巻 UNIT 03 |
| 並行（concurrency） | コンカレンシー。複数の仕事を切り替えながら少しずつ進めること | 第5巻 UNIT 01 |
| 平文（へいぶん） | ハッシュにも暗号化にもしていない、そのままの文字 | 第8巻 UNIT 02 |
| 並列（parallelism） | パラレリズム。複数の CPU で、本当に同時に動かすこと | 第5巻 UNIT 01・02 |
| ヘッダー | 「名前: 値」の行の並び。空行で終わる | 第6巻 UNIT 01、第11巻 UNIT 03 |
| ヘルスチェック | アプリが元気かを外から確かめる仕組み。RustNote は `GET /health`。失敗なら 503 を返す | 第7巻 UNIT 10、第9巻 UNIT 10、第12巻 UNIT 06 |
| 変数 | 値に付けた名前。`let` で作る | 第2巻 UNIT 01 |
| 冪等 | べきとう。同じ操作を1回しても何回しても、結果が同じになる性質 | 第5巻 UNIT 08、第7巻 UNIT 06、第9巻 UNIT 07、第10巻 UNIT 05、第13巻 UNIT 02 |
| 冪等キー | 操作ごとの整理番号。同じキーの2回目を、受ける側が見分けられる | 第5巻 UNIT 08 |
| 冪等性（べきとうせい） | 同じ操作を何回くり返しても、1回したときと結果が変わらない性質 | 第10巻 UNIT 02 |
| ベンチマーク | 同じ処理を何回もくり返して、1回あたりの時間を測ること | 第11巻 UNIT 08 |
| ペルソナ | アプリを使う人を、具体的な1人の人物として思い描いたもの | 第9巻 UNIT 01 |
| ページ送り | `LIMIT` と `OFFSET` で、一覧を少しずつ表示すること | 第7巻 UNIT 02 |
| ホームフォルダ | `C:\Users\（あなたの名前）`。PowerShell では `~` | 第1巻 UNIT 01 |
| ポート | 1台のコンピュータの中で、どのプロセスに届けるかを決める番号 | 第0巻 UNIT 06 |
| ポーリング | 決まった間隔で「仕事は無いか」を見に行くやり方 | 第5巻 UNIT 10 |

### ま行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| マイグレーション | テーブルの形を変える手順を、順番に記録して適用するしくみ | 第0巻 UNIT 07、第9巻 UNIT 07、第13巻 UNIT 02 |
| マクロ | `println!` のように `!` が付く、コードを組み立てる仕組み | 第2巻 UNIT 01 |
| 守りのヘッダー | `X-Frame-Options`・`Content-Security-Policy` など、ブラウザに守り方を伝えるヘッダー | 第11巻 UNIT 03 |
| ミドルウェア | すべての要求の前後に決まった処理をはさむ仕組み。Topcoat は、代わりに `cx` を受け取る関数を勧める | 第6巻 UNIT 07 |
| ミューテーションテスト | わざと間違いを入れて、テストが気づくかを道具で自動に確かめる方法 | 第11巻 UNIT 01、第11巻 UNIT 06 |
| メソッド（HTTP） | リクエストで「何をしてほしいか」を表す言葉（GET・POST） | 第0巻 UNIT 05、第2巻 UNIT 07、第6巻 UNIT 01 |
| メトリクス | リクエスト数・エラー数・応答時間など、数で表した集計 | 第5巻 UNIT 09、第11巻 UNIT 07 |
| メモ化（`#[memoize]`） | 1つの要求の中で、同じ引数の関数の結果を覚えておき、2回目からは計算しない仕組み | 第6巻 UNIT 07 |
| メモリ | 作業中のデータを置く、速いが電源を切ると消える記憶装置 | 第0巻 UNIT 01 |
| 網羅性 | `match` がすべての値を扱っていること | 第2巻 UNIT 04 |
| 文字コード | 文字と数値の対応表。UTF-8・Shift_JIS など | 第0巻 UNIT 03 |
| モジュール | コードをまとめる部屋。既定は非公開 | 第2巻 UNIT 10 |
| モジュールルーティング | モジュールの並びから URL を決める書き方。`module_router!()` を呼んだモジュールが `/` になる | 第6巻 UNIT 03、第13巻 UNIT 07 |
| 持ち主の条件 | `WHERE id = ? AND user_id = ?` のように、ログイン中の人の行だけを対象にする条件 | 第7巻 UNIT 03、第10巻 UNIT 01 |

### や行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| ユニットテスト | 関数や型のような小さな部品1つを、ほかから切り離して確かめるテスト | 第11巻 UNIT 01 |
| ユーザーストーリー | 利用者の立場から「誰として・何をしたい・なぜ」を1文で書いたもの | 第9巻 UNIT 01 |
| ユーザーの列挙 | 応答の違い（文・状態・時間）から、どのメールが登録済みかを調べること | 第13巻 UNIT 06 |
| ユースケース | アプリが利用者のためにする1つの仕事（ノートを作る、など） | 第4巻 UNIT 02 |
| 要求（request） | サーバーへの頼み。1行目（メソッド・パス・版）・ヘッダー・空行・本文でできている | 第6巻 UNIT 01 |
| 要件 | 作るものが満たすべきことの全部。機能・機能以外の約束・作らないことを含む | 第9巻 UNIT 01 |
| 容量 | 1台のマシンで受け付けられる量。CPU・メモリ・DB の、先に足りなくなるもので決まる | 第11巻 UNIT 09 |
| 横の権限・縦の権限 | 他人のデータに届く問題（横）と、管理者の操作に届く問題（縦） | 第8巻 UNIT 07 |
| 横のはみ出し | 中身が画面の幅より広く、横に動かさないと見えないこと | 第10巻 UNIT 10 |
| 読み込み専用トークン | `--read-only` で作る、読むだけで書けないトークン。集計や写しを取るだけのツールに渡す | 第7巻 UNIT 10 |
| 読んだ行・書いた行 | Turso の費用を決める量。SQL が読んだ行の数と、書いた行の数 | 第12巻 UNIT 09 |

### ら行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| 楽観的ロック | 鍵をかけずに編集させ、保存するときに `version` で食い違いを見つける方法 | 第7巻 UNIT 05、第10巻 UNIT 04、第13巻 UNIT 05 |
| ランタイム（executor） | Future を poll して動かす係。実行係 | 第5巻 UNIT 01・02 |
| ランドマーク | `header`・`nav`・`main` など、画面の大きな区切り。同じ種類が2つあるときは名前を付ける | 第11巻 UNIT 05 |
| リクエスト | クライアントからサーバーへのお願い | 第0巻 UNIT 05 |
| リバースプロキシ | アプリの手前で、リクエストを受けて渡す中継のサーバー | 第8巻 UNIT 08 |
| リポジトリ | Git が記録をしまう場所。フォルダの中の `.git` | 第1巻 UNIT 02、第13巻 UNIT 05 |
| リンカー | 部品をつないで実行ファイルを作る道具。Windows では `link.exe`（UNIT 05） | 第1巻 UNIT 01 |
| リージョン | Fly.io のマシンを置く場所。3文字の名前（nrt は東京） | 第12巻 UNIT 04、第12巻 UNIT 09 |
| ルーター | 要求の URL とメソッドを見て、どの関数に処理させるかを決める仕組み | 第6巻 UNIT 02 |
| ルーティング | メソッドとパスによって、処理を振り分けること | 第0巻 UNIT 06 |
| ルートの一覧 | URL ごとの約束（メソッド・誰が・期待する返事）をまとめた表 | 第9巻 UNIT 08 |
| レイテンシ | 1回の往復にかかる時間。距離が遠いほど長くなる | 第12巻 UNIT 09 |
| レイヤー | イメージを作る1つ1つの手順の結果。同じ手順・同じ材料なら、前のビルドの結果（キャッシュ）を使い回す | 第12巻 UNIT 02 |
| レスポンス | サーバーからの返事 | 第0巻 UNIT 05 |
| 列挙型 | 候補のうちどれか1つの値をとる型 | 第2巻 UNIT 08 |
| レベル | ログの重要度。ERROR・WARN・INFO・DEBUG・TRACE | 第5巻 UNIT 09 |
| ログ | プログラムが、何が起きたかを書き残す記録 | 第5巻 UNIT 09 |
| 論理削除（ソフトデリート） | 行を消さずに `deleted_at` などの印を付けて、消したものとして扱うこと | 第10巻 UNIT 06 |
| ローカル / リモート | 手元のファイルの DB を開く形（`new_local`）／ネットの向こうの DB に毎回問い合わせる形（`new_remote`） | 第7巻 UNIT 08 |

### わ行

| 用語 | 意味 | 出てくる所 |
|---|---|---|
| ワイヤーフレーム | 色や飾りを付けずに、画面の部品の配置だけを線で描いた設計図 | 第9巻 UNIT 02 |
| ワイルドカード | `LIKE` の `%`（何文字でも）と `_`（1文字） | 第7巻 UNIT 02 |
| わざと間違えた版 | テストが間違いを見つけられるかを確かめるために、わざと壊した関数 | 第11巻 UNIT 01 |
| ワーカースレッド | ランタイムがタスクを進めるために用意したスレッド | 第5巻 UNIT 02 |
| ワークスペースの設定 | そのフォルダだけに効く設定。`.vscode/settings.json` | 第1巻 UNIT 03 |
