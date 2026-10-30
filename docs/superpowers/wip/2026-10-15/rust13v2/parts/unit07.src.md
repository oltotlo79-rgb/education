## UNIT 07: 画面の枠・部品・登録とログインの画面

### この単元のゴール

- `src/routes.rs` のモジュールルーティング（ファイルの並び＝URL の並び）と、ルーターの組み立て・アクセスログ（`#[layer]`）・画面の枠（`#[layout]`）の役目を説明できる
- `src/views.rs` の部品（`#[component]`）と `view!` の書き方、`(式)` が自動でエスケープされることを、本物の HTML で確かめられる
- エラーが `error_boundary` から `error_page` に届き、状態コード付きの画面になる流れ（404・400・401・409）を、本物のサーバーの応答で言える
- 登録・ログイン・ログアウト・ヘルスチェックの4つのルートの決まり（全部のまちがいを一度に出す・401 は理由を区別しない・ログアウトは POST だけ）を説明できる

### たとえ話でつかむ

デパートの建物を考えます。

- **フロアの案内図**（モジュールルーティング）は、建物の階と売り場の並びが、そのまま案内になっています。「5階の、本の売り場」のように、ファイルの並びがそのまま URL です。
- **入口の受付**（`#[layer]`）は、入ってきたお客さんと、出ていく時刻を記録します。ただし、**守衛さん**（Topcoat の OriginPolicy）が建物の外で止めたお客さんは、受付まで来ないので、記録に残りません。
- どの売り場も、**同じ内装**（`#[layout]`：ヘッダーと外枠）の中にあります。売り場で何か起きたら、**フロアの係**（`error_boundary`）が、「この売り場は見つかりません（404）」のような張り紙（`error_page`）に替えます。
- 張り紙や値札は、**決まった型の部品**（`#[component]`）で作ります。部品に必要な情報（引数）を書き忘れると、作れません。
- 申込書（フォーム）の不備は、**全部まとめて**赤で書いて返します。書き直してもらうとき、合言葉（パスワード）の欄は、空に戻します。

| デパート | RustNote（`src/routes.rs`・`src/views.rs`・`src/routes/`） |
|---|---|
| フロアの案内図 | `module_router!()`（`routes/notes/id/edit.rs` は `/notes/{id}/edit`） |
| 入口の受付の記録 | `#[layer] access_log`（「GET /notes -> 200 OK (4.8ms)」） |
| 建物の外の守衛 | Topcoat の OriginPolicy（ほかのサイトからの POST は 403） |
| 同じ内装 | `#[layout] root_layout`（HTML の外枠・ヘッダー） |
| フロアの係と張り紙 | `error_boundary` と `error_page` |
| 決まった型の部品 | `#[component]`（`site_header`・`flash_banner`・`field_error` など） |
| 不備を全部まとめて返す | `register` の検査（問1） |

> **用語:** レイアウト（layout：どのページにも共通の外枠。RustNote では、HTML の `<head>`、ヘッダー、`<main>` の枠と、エラー画面への差し替え）

<!-- -->

> **用語:** コンポーネント（component：画面の部品。`#[component]` を付けた関数で、`view!` の中から `site_header(user: user)` のように、名前付きの引数で使う）

### 図で理解する

1つのリクエストが、ルーターの中を通る順番です（例7-4・問2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0701" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="110" height="44" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="65" y="47" font-size="12" fill="#0f172a" text-anchor="middle">リクエスト</text>
<rect x="150" y="20" width="150" height="44" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="225" y="39" font-size="11" fill="#b91c1c" text-anchor="middle">Topcoat が先に見る</text>
<text x="225" y="55" font-size="10" fill="#b91c1c" text-anchor="middle">メソッド・OriginPolicy</text>
<rect x="330" y="20" width="120" height="44" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="390" y="39" font-size="12" fill="#c2410c" text-anchor="middle">#[layer]</text>
<text x="390" y="55" font-size="10" fill="#c2410c" text-anchor="middle">access_log</text>
<rect x="480" y="20" width="110" height="44" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="535" y="39" font-size="12" fill="#1d4ed8" text-anchor="middle">#[layout]</text>
<text x="535" y="55" font-size="10" fill="#1d4ed8" text-anchor="middle">root_layout</text>
<rect x="620" y="20" width="90" height="44" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="665" y="39" font-size="12" fill="#047857" text-anchor="middle">#[page]</text>
<text x="665" y="55" font-size="10" fill="#047857" text-anchor="middle">signup など</text>
<line x1="120" y1="42" x2="146" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0701)"/>
<line x1="300" y1="42" x2="326" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0701)"/>
<line x1="450" y1="42" x2="476" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0701)"/>
<line x1="590" y1="42" x2="616" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0701)"/>
<text x="150" y="88" font-size="11" fill="#b91c1c">GET /logout → 405（allow: POST）</text>
<text x="150" y="104" font-size="11" fill="#b91c1c">ほかのサイトからの POST → 403</text>
<text x="150" y="120" font-size="11" fill="#b91c1c">（access_log まで来ないので、記録に残らない）</text>
<rect x="480" y="140" width="230" height="110" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="492" y="162" font-size="11" fill="#1d4ed8">ページの結果</text>
<text x="492" y="182" font-size="11" fill="#047857">Ok(画面) → 枠の中に入れて 200 など</text>
<text x="492" y="202" font-size="11" fill="#c2410c">Err(see_other) → そのまま 303</text>
<text x="492" y="222" font-size="11" fill="#b91c1c">Err(AppError など) → error_boundary</text>
<text x="492" y="240" font-size="11" fill="#b91c1c">　→ error_page（404・400・500 の画面）</text>
<line x1="665" y1="64" x2="665" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0701)"/>
<rect x="10" y="140" width="440" height="110" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="22" y="162" font-size="11" fill="#0f172a">モジュールルーティング（ファイルの並び ＝ URL の並び）</text>
<text x="22" y="184" font-size="11" fill="#475569">routes.rs → /　　routes/signup.rs → /signup</text>
<text x="22" y="204" font-size="11" fill="#475569">routes/notes.rs → /notes　　routes/notes/id.rs → /notes/{id}</text>
<text x="22" y="224" font-size="11" fill="#475569">routes/notes/id/edit.rs → /notes/{id}/edit</text>
<text x="22" y="242" font-size="11" fill="#475569">どこにも当たらない → not_found!() → 404 の画面</text>
<text x="10" y="284" font-size="11" fill="#0f172a">access_log は、ページの結果を見てから「POST /signup -> 303 See Other (32.8ms)」と書く。</text>
<text x="10" y="304" font-size="11" fill="#0f172a">GET /health だけは、わざと書かない（数十秒ごとに来るため）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: リクエストの通り道。メソッドの違いとほかのサイトからの POST は、Topcoat が先に止める。ページのエラーは、枠の中の error_boundary が画面に変える</div>
</div>

登録（`POST /signup`）の処理の分かれ道です（例7-4・問1の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0702" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="150" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="85" y="44" font-size="12" fill="#1d4ed8" text-anchor="middle">① 入力の検査</text>
<text x="85" y="64" font-size="10" fill="#1d4ed8" text-anchor="middle">メール・長さ・確認</text>
<rect x="200" y="20" width="150" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="275" y="44" font-size="12" fill="#1d4ed8" text-anchor="middle">② ハッシュを作る</text>
<text x="275" y="64" font-size="10" fill="#1d4ed8" text-anchor="middle">Argon2id</text>
<rect x="390" y="20" width="150" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="465" y="44" font-size="12" fill="#1d4ed8" text-anchor="middle">③ users に保存</text>
<text x="465" y="64" font-size="10" fill="#1d4ed8" text-anchor="middle">UNIQUE(email)</text>
<rect x="580" y="20" width="130" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="645" y="44" font-size="12" fill="#047857" text-anchor="middle">④ log_in</text>
<text x="645" y="64" font-size="10" fill="#047857" text-anchor="middle">303 /notes（PRG）</text>
<line x1="160" y1="48" x2="196" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0702)"/>
<line x1="350" y1="48" x2="386" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0702)"/>
<line x1="540" y1="48" x2="576" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0702)"/>
<line x1="85" y1="76" x2="85" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0702)"/>
<line x1="465" y1="76" x2="465" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0702)"/>
<rect x="10" y="120" width="300" height="60" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="160" y="144" font-size="12" fill="#b91c1c" text-anchor="middle">400・フォームを再表示</text>
<text x="160" y="164" font-size="10" fill="#b91c1c" text-anchor="middle">問題を全部、欄ごとに出す（パスワードは空に戻す）</text>
<rect x="390" y="120" width="320" height="60" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="550" y="144" font-size="12" fill="#c2410c" text-anchor="middle">UniqueViolation → 409・フォームを再表示</text>
<text x="550" y="164" font-size="10" fill="#c2410c" text-anchor="middle">「そのメールアドレスは既に登録されています」</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 登録の分かれ道。成功したら 303 で一覧へ移り（PRG）、だめならフォームを状態コード付きで再表示する</div>
</div>

### まず動かす最小コード

#### 例7-1: src/routes.rs の全文

> **▼ このファイルがやること（先に日本語で）:** URL ごとの処理（ルート）の、いちばん上のファイルです。子のモジュール（`health`・`login`・`logout`・`notes`・`signup`）を宣言し、ルーターを組み立てる `build_router`、どの URL にも当たらないときの `not_found!()`、アクセスログの `#[layer]`、全部のページを包む `#[layout]`、トップページ（`GET /`）の `#[page]` を置いています。

```rust
@@FILE:rustnote/src/routes.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ファイルの並びが、URL の並び

- `module_router!()` を呼んだこのモジュール（`routes`）が `/` になり、子のモジュールが、URL の区切りを1つずつ足します。`routes/notes/id/edit.rs` は `/notes/{id}/edit` です（先頭のコメントの表）。新しいページを足すときは、ファイルを足して、親に `pub mod` を1行足します（UNIT 03 のエラー1）。
- `build_router` は、ルーターに、クッキー（`.cookies()`）・セッション（`.sessions(SessionConfig::default())`）・CSS などのアセット（`.assets(assets)`）・共有の状態（`.app_context(state)`）を足してから、`.build()` で仕上げます。`main.rs` も、テスト（`tests/common/mod.rs`）も、この同じ関数を使います。
- `not_found!()` は、どのルートにも当たらない URL を `NotFoundError` にして、画面の枠の `error_boundary` まで届けます。だから、`/nope` も、ほかのページと同じ枠の中の「404 Not Found」の画面になります（例7-4）。

##### 解説2: アクセスログは #[layer]

- `#[layer]` の関数は、ページの処理を `next.run(cx, body)` で呼ぶ前と後に、処理を足せます。`access_log` は、時刻を測り、結果の状態を「`POST /signup -> 303 See Other (32.8ms)`」の形で標準エラー出力に書きます。
- ページが `Err(see_other(…))` で返したリダイレクトは、この時点ではまだ `Err` に見えるので、`SeeOther` かどうかを `downcast_ref` で見て、「303 See Other」と書きます。
- `/health` は、Fly.io の見張りから数十秒ごとに来るので、書きません。
- **注意:** Topcoat が `#[layer]` より前に止めたリクエスト（メソッドが違う 405・ほかのサイトからの POST の 403）は、ここまで来ないので、記録に残りません（問2）。

##### 解説3: 画面の枠は #[layout]

- `root_layout` は、どのページも包む HTML の外枠です。`<head>`・ヘッダー（`site_header`）・`<main class="container">` を書き、ページの中身は `(slot)` の所に入ります。
- ヘッダーの「ログイン中のメール」のために `current_user` を呼びますが、`.ok().flatten()` で、DB の失敗を「ログインしていない」と同じに扱います。ヘッダーのために枠ごと失敗して、エラー画面すら出せなくなるのを防ぐためです。
- `error_boundary(fallback: |error| error_page(cx, error), (slot))` は、ページで起きたエラーを受け止め、`error_page` で画面に変えます。
- `topcoat::dev::script()` は、`topcoat dev` で動かしているときだけ、自動で再読み込みするためのスクリプトを入れます。`topcoat asset bundle` で作った実行ファイルでは、何も入りません（例7-4の HTML に、スクリプトはありません）。

#### 例7-2: src/views.rs の全文

> **▼ このファイルがやること（先に日本語で）:** 画面の部品と、エラー画面です。ヘッダー（`site_header`）・お知らせ（`flash_banner`）・入力欄の下のエラー文（`field_error`）・ノートのフォーム（`note_form`）・ページ送り（`pagination`）の部品と、エラーを状態コード付きの画面にする `error_page` があります。

```rust
@@FILE:rustnote/src/views.rs@@
```

##### 解説4: view! の書き方と、自動のエスケープ

- `view!` の中では、HTML のタグをそのまま書き、文字は `"…"` で囲みます。囲まずに書くと、マクロが読めずにコンパイルエラーになります（エラー1）。
- `(式)` の所には、Rust の値を入れます。値は**自動で HTML エスケープ**されるので、利用者が入力した `<b>` や `<script>` は、`&lt;b&gt;` のように文字として表示されるだけで、タグとしては働きません（問3）。
- `if let … { … } else { … }` も、`view!` の中にそのまま書けます。`site_header` は、ログイン中ならメールとログアウトのボタン、そうでなければログインと新規登録のリンクを出します。
- `href!(crate::routes::login::login_page)` は、ページの関数から URL（`/login`）を作ります。URL を文字列で書かないので、ページを移したり名前を変えたりすると、コンパイルエラーで気づけます。ページでない関数を渡すと、エラーになります（エラー3）。
- `STYLESHEET` は `asset!("assets/style.css")` です。配信される URL は `/_topcoat/assets/style-e9b983efabcfc6c3.css` のように、中身から作った印（ハッシュ）が付きます（UNIT 09）。

##### 解説5: 部品（#[component]）

- `#[component]` を付けた関数は、`view!` の中から `flash_banner(flash: flash)` のように、**名前付きの引数**で使います。引数を書き忘れると、「`flash` が設定されていません」というコンパイルエラーになります（エラー2）。
- `note_form` と `pagination` は、ノートの画面で使います（UNIT 08）。`note_form` は、`form.version` があれば（編集のとき）隠し項目 `version` を入れ、楽観的ロックの版の番号をフォームと一緒に送らせます（UNIT 05）。

##### 解説6: error_page は、エラーの種類を見分ける

- `error.downcast_ref::<AppError>()` のように、`topcoat::Error` の中身の型を順に調べます。`AppError` なら `status_code` と `public_message`、`RepoError` なら同じ分類の表（UNIT 04 の改善点）、Topcoat の `NotFoundError` なら 404、`BadRequestError`（フォームの形が違う など）なら 400 です。
- どれでもないエラー（303 のリダイレクトなど）は、`return Err(error)` で投げ直し、Topcoat に任せます。
- 普通の関数の中で `view!` を使うときは、先頭に `cx =>` を書いて、文脈を渡します。`(status)` を `view!` の先頭に書くと、その状態コードで返します。
- `log_if_internal` は、500 のときだけ、エラーの原因の鎖（`error.chain()`）を「`[error] … <- …`」の形でログに書きます。画面には `public_message` しか出しません。

#### 例7-3: 4つのルートの全文

> **▼ このファイルがやること（先に日本語で）:** `signup.rs` は登録のフォーム（`GET /signup`）と登録（`POST /signup`）、`login.rs` はログインのフォームとログイン、`logout.rs` はログアウト（`POST /logout` だけ）、`health.rs` は動いているかの確認（`GET /health`）です。

```rust
@@FILE:rustnote/src/routes/signup.rs@@
```

```rust
@@FILE:rustnote/src/routes/login.rs@@
```

```rust
@@FILE:rustnote/src/routes/logout.rs@@
```

```rust
@@FILE:rustnote/src/routes/health.rs@@
```

##### 解説7: 登録は、全部のまちがいを一度に出す

- `Form(input): Form<SignupForm>` は、送られてきたフォームを、`serde` で `SignupForm` に読み取ります。項目が足りないなど、形が違えば、ページの処理に入る前に `BadRequestError`（400「送信された内容を読み取れませんでした」）になります（例7-4）。
- `register` は、メール・パスワード・確認の3つを、途中で止めずに全部調べ、問題を `SignupFormState` の欄ごとに入れます（問1）。`let (Some(email), Some(password), None) = (…) else { … }` は、「3つともそろったときだけ先へ進む」を1つの形で書いています。
- 検査に通ったら、ハッシュを作って保存します。`UniqueViolation` なら 409 でフォームを再表示し、成功したら `log_in` して、`flash::set` と 303 で一覧へ移ります。POST の結果を 303 で GET の画面に移す形を、PRG（Post/Redirect/Get）と言います。画面を再読み込みしても、登録がもう一度送られません。
- フォームを再表示するとき、メールは `value=(form.email)` で戻しますが、パスワードは戻しません（`SignupFormState` にパスワードの欄が無い）。パスワードを HTML に書き出さないためです。

##### 解説8: ログイン・ログアウト・ヘルスチェック

- `login` は、`authenticate` が `None` なら、メールが無いのかパスワードが違うのかを区別せず、同じ文と 401 で返します（UNIT 06）。メールの欄には、入力を戻します。
- ログイン済みの人が `/login`・`/signup`・`/` を開くと、303 で `/notes` へ移ります。
- `logout` は `#[page(POST)]` だけです。状態を変える操作を GET にすると、画像のタグ（`<img src="/logout">`）などで、知らないうちにログアウトさせられるからです。ほかのサイトからの POST は、Topcoat の OriginPolicy が 403 で止めます（問2）。
- `health` は `#[route(GET)]` で、HTML の枠を通らずに JSON を返します。DB に `SELECT 1` ができれば `200 {"status":"ok"}`、できなければ `503 {"status":"unavailable"}` です。Fly.io は、これで「動いているか」を見張ります（第12巻）。

### もう一歩: 実務に近い形

#### 例7-4: 本物のサーバーで、画面とエラーを確かめる

`topcoat asset bundle --release` で作った実行ファイルを、一時フォルダで `PORT=3014`・`DATABASE_URL=u07.db` で起動し、`curl` で送りました。表示は、状態の行と主なヘッダー（`set-cookie` のトークンは先頭の 6 文字だけ）と、本文の一部です。本文は1行で返ってくるので、見やすいように、タグの区切り（`><`）で改行しました。

**▼ トップページ（`GET /`、ログインしていない）の HTML 全体**

```text
@@OUT:u07_home@@
```

**▼ 無い URL（`GET /nope`）の `<main>` の中**

```text
@@OUT:u07_404@@
```

**▼ 項目の足りないフォーム（`POST /login` に `email=a` だけ）の `<main>` の中**

```text
@@OUT:u07_400@@
```

**▼ 登録（`POST /signup`）と、そのクッキーで開いた `/notes` のヘッダー**

```text
@@OUT:u07_signup_ok@@
```

```text
@@OUT:u07_notes_header@@
```

**▼ 同じメールで、もう一度登録した**

```text
@@OUT:u07_signup_dup@@
```

**▼ パスワードを間違えてログインした**

```text
@@OUT:u07_login_fail@@
```

**▼ ヘルスチェック**

```text
@@OUT:u07_health@@
```

- トップページは、`root_layout` の枠（`<head>`・ヘッダー・`<main>`）の中に、`home` の中身（`<section class="hero">`）が入っていました。どのページも、`<title>` は「RustNote」です。
- `/nope` と項目の足りないフォームは、同じ枠の中の `error_page` の画面（404・400）になりました。
- 登録のあと、ヘッダーは「ノート一覧・メール・ログアウトのボタン」に変わりました。ログアウトのボタンは、`POST /logout` のフォームです。
- 同じメールの2回目は 409 で、メールの欄の下にエラー文が出て、メールは入力が戻り、パスワードの欄は空でした。ログインの失敗は 401 で、フォームの上に「メールアドレスまたはパスワードが違います」が出ました。

### よくあるエラーと読み方

#### エラー1: `view!` の中の文字を、`"…"` で囲まずに書いた

```rust
@@FILE:errs_u07/e07_text.rs@@
```

```text
@@OUT:u07_err_text@@
```

- **何が起きたか:** 「view の部品（ノード）が来るはずです」というエラーです。`view!` の中では、`RustNote` は文字ではなく、Rust の名前のように読まれます。
- **直し方:** `<h1>"RustNote"</h1>` と、文字を `"…"` で囲みます。

#### エラー2: 部品を、引数を渡さずに使った（E0277）

```rust
@@FILE:errs_u07/e07_prop.rs@@
```

```text
@@OUTHEAD:u07_err_prop:8@@
  …（部品の作り方の説明が続く）…
@@OUTGREP:u07_err_prop:^error: could not@@
```

- **何が起きたか:** 「必要な引数 `flash` が設定されていません」というエラーです。`#[component]` は、引数を、名前で1つずつ設定する形（ビルダー）に変えます。`#[default]` の付いていない引数は、全部設定しないと、部品を作れません。
- **直し方:** `flash_banner(flash: flash)` のように、引数を名前付きで渡します。

#### エラー3: `href!` に、ページではない関数を渡した（E0277）

```rust
@@FILE:errs_u07/e07_href.rs@@
```

```text
@@OUTHEAD:u07_err_href:16@@
  …（同じ形のエラーが、もう1つ続く）…
@@OUTGREP:u07_err_href:^error: could not@@
```

- **何が起きたか:** 「`site_header` は `HrefTarget`（リンクの行き先）を実装していません」というエラーです。`help:` に、行き先にできるもの（`home`・`login_page` など、ページの関数）が並んでいます。
- **読み方:** `href!` は、ページの関数から URL を作るので、部品（`site_header`）は行き先にできません。URL の打ち間違いが、実行する前に見つかるのは、このしくみのおかげです。

### RustNote ではここで使う

- **ルーターの組み立て:** `src/main.rs` が `build_router(AppState::new(db), assets)` を呼び、`topcoat::start` に渡します（UNIT 03）。テストは、同じ `build_router` に、テスト用の DB とアセットの設定を渡します（UNIT 10）。
- **部品:** `note_form`・`pagination`・`field_error`・`flash_banner` は、ノートの画面でも使います（UNIT 08）。
- **テスト:** `tests/http.rs` の 14 のテストのうち、この単元の分は6つです（UNIT 10）。

```text
@@OUTGREP:u07_http_tests:^(running|test |test result)@@
```

- **改善点（この単元で見つかったこと）:**
  - どのページも `<title>` が「RustNote」だけです。ブラウザのタブや、画面の読み上げで、どのページか分かりません。
  - ヘッダーの `<nav>` と、ページ送りの `<nav>` に名前（`aria-label`）が無く、アクセシビリティの検査（axe）で「同じ種類の目印は区別できるように」と指摘されました（第11巻 UNIT 05）。
  - アクセスログに、Topcoat が先に止めた 405・403 が残りません（問2）。

### 練習問題

**問1:** 登録のフォームで、メール（`alice`）・パスワード（`short`）・確認（`different`）を3つともまちがえて送ると、何が返るでしょうか。

**問2:** `GET /logout` と、ほかのサイトから（`Origin: https://evil.example` や `Sec-Fetch-Site: cross-site` を付けて）の `POST /logout` は、どうなるでしょうか。同じサイトから（`Origin: http://127.0.0.1:3014`）の `POST /logout` と比べてください。アクセスログには、どう残るでしょうか。

**問3:** メールを `<b>bold</b>@example.com` にして登録すると、ヘッダーのメールの表示は、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
@@OUT:u07_q1@@
```

- 400 で、3つの欄の下に、それぞれのエラー文が一度に出ました。メールは入力（`alice`）が戻り、パスワードの2つの欄は空です。
- サーバーのログでは、この `POST /signup -> 400` は 0.1 ミリ秒ほどでした（下のログ）。入力の検査で止まったので、時間のかかる Argon2id まで進んでいないからです。

**問2の解答:**

```text
@@OUT:u07_q2_get@@
```

```text
@@OUT:u07_q2_origin@@
```

```text
@@OUT:u07_q2_fetch@@
```

```text
@@OUT:u07_q2_same@@
```

- `GET /logout` は 405 で、`allow: POST`（POST だけ受け付ける）と返りました。
- `Origin` がほかのサイトのときも、`Sec-Fetch-Site: cross-site`（ブラウザが付ける「ほかのサイトから」の印）のときも、Topcoat の OriginPolicy が 403 の `forbidden` で止め、ログアウトは起きませんでした。同じサイトからのときだけ、303 でログアウトしました。

この例7-4・問1〜3の間の、サーバーのログの全部です。

```text
@@OUT:u07_server_log@@
```

- サーバーに届いたリクエストは 16 回（起動を確かめた `/health` を含む）ですが、ログは 11 行です。405 と 2 回の 403 は、Topcoat が `access_log`（`#[layer]`）より前で止めたので、記録に残りませんでした。`/health` の 2 回は、わざと書いていません。
- 攻撃の試み（ほかのサイトからの POST）は、このログには残りません。このログだけを見て「攻撃は来ていない」と判断しないよう、気をつけます。

**問3の解答:**

```text
@@OUT:u07_q3_header@@
```

- メールは `&lt;b&gt;bold&lt;/b&gt;@example.com` と、エスケープされて出ました。ブラウザでは `<b>bold</b>@example.com` という文字のまま表示され、太字にはなりません。`view!` の `(式)` が、自動で `<` を `&lt;` にするからです。
- `Email::parse` は、`@` が1つでドメインに `.` があれば通すので、このメールは登録できました。画面に出す所で必ずエスケープされるので、入力の検査で記号を断らなくても安全です。

### まとめ

- `src/routes.rs` は、`module_router!()` でファイルの並びを URL にし、`build_router` でクッキー・セッション・アセット・共有の状態を足す。`#[layer] access_log` が「`POST /signup -> 303 See Other`」を書き（405・403 は来ない）、`#[layout] root_layout` が枠と `error_boundary` を持つ。
- `src/views.rs` の `view!` は文字を `"…"` で囲み（囲まないと expected view node）、`(式)` を自動でエスケープする。`#[component]` は名前付きの引数（忘れると E0277）。`href!` はページの関数から URL を作る（ページでないと E0277）。`error_page` は型を見分けて 404・400・409・500 の画面にする。
- 登録は全部のまちがいを一度に 400 で出し、登録済みは 409、成功は PRG の 303。ログインの失敗は理由を区別しない 401。ログアウトは POST だけで、ほかのサイトからは 403。ヘルスチェックは JSON の 200 か 503。
