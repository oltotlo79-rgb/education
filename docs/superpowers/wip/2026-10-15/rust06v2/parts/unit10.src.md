## UNIT 10: エラー・クッキー・セッション・CSRF

### この単元のゴール

- ページで起きた 404・400 を、`not_found!()` とレイアウトの `error_boundary` で、**状態コード付きの自分の画面**に変えられる。リダイレクトは変えずに通す理由を言える
- `cookies(cx)` でクッキーを読み書きし、`Set-Cookie` の中身（`Path`・`Max-Age` など）を読める。署名付きのクッキーで、書き換えを見つけられる
- `session::start`・`token_hash`・`stop` で、ログインの土台（トークンを発行し、ハッシュで記録し、終わらせる）を作れる
- CSRF（ほかのサイトから、ログイン中の人に操作させる攻撃）を、Topcoat の OriginPolicy がどう止めるかを確かめ、状態を変える操作を GET にしてはいけない理由を説明できる

### たとえ話でつかむ

会員制のスポーツクラブを考えます。

- 受付の人は、困ったことが起きたとき、**決まった案内の紙**を渡します。「その部屋はありません」「書き方が違います」。どの係で起きても、同じ形の紙です。ただし、「2階へどうぞ」という**道案内**は、困ったことではないので、そのまま伝えます。
- 会員には、**入館証**を渡します。入館証には、番号だけが書いてあります。クラブは、番号の**写し（ハッシュ）**を台帳に書いておき、入館証を見せられたら、台帳と照らし合わせます。退会や帰るときは、入館証を返してもらい、台帳からも消します。
- 来た人の上着に付ける**札**（「今日はロッカー 12 番」）は、本人が書き換えられます。大事な札には、**クラブの印**を押しておき、印の無い札は無視します。
- 外の誰かから「この会員の退会の手続きをして」という紙が届いても、**会員本人が窓口に来た**のでなければ、受け付けません。

| スポーツクラブ | Topcoat |
|---|---|
| 決まった案内の紙 | `error_boundary` の中で作る、エラーの画面（404・400） |
| 道案内はそのまま | リダイレクト（303・307）は、エラーの画面にせず Topcoat に返す |
| 入館証（番号だけ） | セッションのトークン（`__Host-session` のクッキー） |
| 台帳の写し | トークンのハッシュ（`TokenHash`）と、その人の記録 |
| 上着の札 | ふつうのクッキー（`theme=dark` など）。利用者が書き換えられる |
| クラブの印 | 署名付きのクッキー（`signed_cookies`）。印が合わなければ読まない |
| 本人が窓口に来たか | OriginPolicy（ほかのサイトから送られた POST を 403 で止める） |

> **用語:** クッキー（cookie：サーバーが `Set-Cookie` で預け、ブラウザが次の要求から `Cookie` で送り返してくる、小さな名前と値の組）

<!-- -->

> **用語:** セッション（session：一度ログインした人を、次の要求からも同じ人と分かるようにする仕組み。第8巻 UNIT 01）

<!-- -->

> **用語:** CSRF（シーサーフ：ほかのサイトのページから、ログイン中の人のブラウザに、勝手に要求を送らせる攻撃。クッキーは自動で付くので、本人の操作に見えてしまう）

### 図で理解する

ページで起きたエラーが、画面になるまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-1001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="190" height="90" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="115" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ページが Err を返す</text>
<text x="115" y="66" font-size="11" fill="#475569" text-anchor="middle">ok_or_not_found()?</text>
<text x="115" y="84" font-size="11" fill="#475569" text-anchor="middle">bad_request(…)・redirect(…)</text>
<text x="115" y="102" font-size="11" fill="#475569" text-anchor="middle">ほかのエラー（? で）</text>
<rect x="250" y="20" width="220" height="90" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">レイアウトの error_boundary</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">fallback: error_page(cx, error)</text>
<text x="360" y="86" font-size="11" fill="#0f172a" text-anchor="middle">downcast_ref で種類を見分ける</text>
<rect x="510" y="10" width="190" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="32" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">知っている種類</text>
<text x="605" y="52" font-size="11" fill="#0f172a" text-anchor="middle">404・400 → 自分の画面</text>
<rect x="510" y="76" width="190" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="605" y="98" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">知らない種類</text>
<text x="605" y="118" font-size="11" fill="#0f172a" text-anchor="middle">Err(error) で Topcoat に返す</text>
<line x1="210" y1="65" x2="246" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<line x1="470" y1="50" x2="506" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<line x1="470" y1="80" x2="506" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<rect x="250" y="160" width="450" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="475" y="184" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Topcoat が返事にする</text>
<text x="475" y="206" font-size="11" fill="#0f172a" text-anchor="middle">リダイレクト（redirect・see_other）→ 307・303 と location</text>
<text x="475" y="226" font-size="11" fill="#b91c1c" text-anchor="middle">ルーターのエラーではないもの → 500「internal server error」</text>
<line x1="605" y1="132" x2="605" y2="156" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<rect x="20" y="160" width="190" height="80" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="115" y="184" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">どこにも当たらない URL</text>
<text x="115" y="206" font-size="11" fill="#0f172a" text-anchor="middle">not_found!() があれば</text>
<text x="115" y="224" font-size="11" fill="#0f172a" text-anchor="middle">NotFoundError としてレイアウトへ</text>
<line x1="210" y1="190" x2="300" y2="112" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: エラーの流れ。ページの Err はレイアウトの error_boundary に届き、知っている種類だけを自分の画面に変える。リダイレクトなど知らない種類は Topcoat に返す</div>
</div>

ログインからログアウトまでの、セッションの一生です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-1002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="210" height="96" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① ログイン（POST /login）</text>
<text x="125" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::start(cx)</text>
<text x="125" y="86" font-size="11" fill="#475569" text-anchor="middle">新しいトークンをクッキーで渡し、</text>
<text x="125" y="104" font-size="11" fill="#475569" text-anchor="middle">ハッシュを記録する</text>
<rect x="255" y="20" width="210" height="96" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">② 次の要求（GET /）</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::token_hash(cx)</text>
<text x="360" y="86" font-size="11" fill="#475569" text-anchor="middle">送られたトークンのハッシュで、</text>
<text x="360" y="104" font-size="11" fill="#475569" text-anchor="middle">記録を探す</text>
<rect x="490" y="20" width="210" height="96" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">③ ログアウト（POST /logout）</text>
<text x="595" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::stop(cx)</text>
<text x="595" y="86" font-size="11" fill="#475569" text-anchor="middle">クッキーを消させ、</text>
<text x="595" y="104" font-size="11" fill="#475569" text-anchor="middle">記録も消す</text>
<line x1="230" y1="68" x2="251" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1002)"/>
<line x1="465" y1="68" x2="486" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1002)"/>
<rect x="20" y="146" width="680" height="76" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="170" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Set-Cookie: __Host-session=（32 バイトの乱数）; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000</text>
<text x="360" y="194" font-size="11" fill="#475569" text-anchor="middle">記録に残すのはトークンのハッシュだけ。記録が盗まれても、ハッシュはトークンとして使えない</text>
<text x="360" y="212" font-size="11" fill="#475569" text-anchor="middle">トークンが1文字でも違えば、ハッシュが違うので、記録は見つからない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: セッションの一生。Topcoat はトークンの発行と受け渡しをし、アプリはハッシュと記録の保存と照合をする。ログアウトでは、クッキーと記録の両方を消す</div>
</div>

### まず動かす最小コード

#### 例10-1: エラーを、状態コード付きの自分の画面にする

> **▼ このコードがやること（先に日本語で）:** `not_found!()` で、どこにも当たらない URL も `NotFoundError` にします。レイアウトでは `error_boundary` でページを包み、エラーが起きたら `error_page` を呼びます。`error_page` は、`NotFoundError` なら 404 の画面、`BadRequestError` なら 400 の画面を作り、それ以外は `Err(error)` で Topcoat に返します。ページは、`/items/{id}`（1〜3 番だけある）・`/search?q=…`（言葉が無ければ 400）・`/old`（トップへ 307 で移す）・`/boom`（ファイルを読めずにふつうのエラー）です。

```rust
@@FILE:web_lab/src/bin/u10_errors.rs@@
```

```powershell
cargo run --bin u10_errors
```

**▼ 実行結果（404）**

```text
@@OUT:u10_404@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 404 も、レイアウトの中の自分の画面になる

- `/items/9` は、`ok_or_not_found()?` で `NotFoundError` になり、`404 Not Found` と、レイアウト（`<header>`）付きの「ページが見つかりません」が返りました。
- `/nothing/here`（どこにも当たらない URL）も、同じ画面です。`not_found!()` を書かないと、こうした URL は**レイアウトを通らずに** `not found` の文字だけになります（UNIT 02・03 の 404）。`not_found!()` は、「ほかに当たらない URL は、全部ここへ」というページを足して、`NotFoundError` をレイアウトまで届けます。
- エラーの画面の最初に `(status)` を置いています。これが無いと、エラーの画面なのに **200** で返ってしまいます（UNIT 06 の 400 と同じ考え方です）。

##### 解説2: 400 は、わけを書いて返す

```text
@@OUT:u10_400@@
```

- `bad_request("検索の言葉を入れてください")` の文は、`BadRequestError` の `description()` で取り出せます。ここで渡した文は、利用者に見えるので、直し方が分かる、短い文にします。

##### 解説3: リダイレクトと、ふつうのエラーは Topcoat に返す

```text
@@OUT:u10_other@@
```

- `/old` は `307 Temporary Redirect` と `location: /` でした。`redirect(…)` も、`error_boundary` には「エラー」として届きますが、`error_page` は知らない種類として `Err(error)` で返したので、Topcoat が 307 の返事にしました（エラー3）。`redirect` は 307（メソッドを変えずに送り直す）、フォームのあとに使う `see_other` は 303 です（UNIT 06）。
- `/boom` は 500 で、本文は `internal server error` だけでした。ファイルが無い、という**ふつうのエラー**（`std::io::Error`）は、ルーターのエラーではないので、Topcoat は中身を見せずに 500 にします。サーバーの窓にも、**何も表示されませんでした**。何が起きたかを知るには、自分で記録します（問1）。

### もう一歩: 実務に近い形

#### 例10-2: クッキーとセッションで、「誰が見ているか」を覚える

> **▼ このコードがやること（先に日本語で）:** `GET /` は、クッキー `theme` と、セッションの名前を表示します。`POST /theme` はテーマのクッキーを書き換え、`POST /login` は `session::start` で新しいセッションを始めて、トークンのハッシュと名前を記録し、`POST /logout` は `session::stop` でセッションを終えて記録を消します。どれも、終わったら 303 でトップへ移ります。ルーターには、`.cookies()`・`.sessions(…)`・記録の置き場（`Sessions`）を登録します。

```rust
@@FILE:web_lab/src/bin/u10_session.rs@@
```

```powershell
cargo run --bin u10_session
```

まず、ふつうのクッキーです。

```text
@@OUT:u10_cookie@@
```

- `POST /theme` の返事に、`set-cookie: theme=dark; Path=/` が付きました。ブラウザは、これを覚えて、次の要求から `Cookie: theme=dark` を付けて送ります（ここでは、curl で自分で付けました）。`jar.add(…)` で予約したクッキーは、ページが 303 を返したときにも、返事に付きます。

次に、セッションです（トークンは、実行するたびに変わります）。

```text
@@OUT:u10_session_flow@@
```

- ログインの返事には、`__Host-session=…` のクッキーが付きました。トークンは 32 バイトの乱数で、`HttpOnly`（JavaScript から読めない）・`SameSite=Lax`（ほかのサイトからの POST には付けない）・`Secure`（HTTPS でだけ送る）・`Path=/`・`Max-Age=2592000`（30 日）が、Topcoat の既定で付きます（第8巻 UNIT 01）。
- そのトークンを送ると「alice」と分かり、**1文字変えると分かりません**でした。記録の鍵は、トークンそのものではなく、そのハッシュ（`TokenHash`）なので、少しでも違うトークンからは、別のハッシュになるからです。
- ログアウトの返事は、`Max-Age=0` と1年前の `Expires` で、クッキーを消させます。そのうえ記録も消したので、**ログアウトの前のトークンを送っても**、もう分かりません。クッキーを消すだけでは、トークンを写し取っておいた人が使えてしまうので、記録も消すのが大事です。

> **注意:** `Secure` のクッキーは、ふつうのブラウザでは HTTPS でしか送られません（`http://127.0.0.1` は、多くのブラウザが例外として扱います）。この例では、curl で自分で `Cookie:` を付けて確かめています。

#### ▼ CSRF を、OriginPolicy が止める

ほかのサイト（`https://evil.example`）のページに、`/theme` へ POST するフォームが仕込まれていた、と考えます。ブラウザは、そのフォームを送るとき、`Origin`（どのサイトのページから送ったか）と `Sec-Fetch-Site: cross-site`（ほかのサイトから）を付けます。curl で、同じヘッダーを付けて試します。

```text
@@OUT:u10_csrf@@
```

- ほかのサイトからの POST は **403** で、テーマは変わりませんでした。同じサイト（`Sec-Fetch-Site: same-origin`）からの POST は通りました。Topcoat のルーターは、すべての要求の最初に OriginPolicy で確かめるので、自分では何も書かなくても、この守りが効いています（RustNote も同じです）。
- **GET は止めません**。ほかのサイトからのリンクで、ページを開くのは、ふつうのことだからです。だから、状態を変える操作（ログアウト・削除・書き換え）を GET にすると、守りが効きません（エラー2）。

### よくあるエラーと読み方

#### エラー1: .cookies() を書き忘れた

`.sessions(…)` だけ書いて、`.cookies()` を書き忘れた場合です（`errs/e10_nocookies.rs`）。

```text
@@OUT:u10_err_nocookies_curl@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u10_err_nocookies_server@@
```

- **何が起きたか:** 「`CookieJarCell` 型の、要求のコンテキストを使おうとしましたが、登録されていません」という panic で、500 になりました。クッキーの入れ物（jar）は、`.cookies()` が要求ごとに用意します。セッションもクッキーでトークンを運ぶので、`.cookies()` が要ります（UNIT 07 のエラー1と同じ形の panic です）。
- **直し方:** `module_router!().cookies().sessions(SessionConfig::default())` と、両方を登録します。RustNote の `build_router` も、この順に書いています（UNIT 03）。

#### エラー2（エラーが出ない間違い）: ログアウトを GET にした

ログアウトを、`<a href="/logout">` のリンクで済ませたくて、`#[page]`（GET）にした場合です（`errs/e10_getlogout.rs`）。ほかのサイトのリンクから開かれた、と考えて試します。

```text
@@OUT:u10_err_getlogout@@
```

- **何が起きたか:** ほかのサイトから開かれた GET なのに、ログアウトが**できてしまいました**。OriginPolicy は GET を止めないからです。`SameSite=Lax` のクッキーも、ほかのサイトのリンクで**ページを開くとき**（GET）には送られるので、本物のブラウザでも、同じことが起きます。ほかのサイトに、あなたのサイトの `/logout` へのリンクを置かれるだけで、押した人がログアウトさせられます（画像の読み込みのような、ページを開く以外の GET には、`SameSite=Lax` のクッキーは付きません）。
- **直し方:** 状態を変える操作は、POST のフォーム（ボタン）にします。RustNote のログアウトも、`#[page(POST)]` です（UNIT 06）。GET は「見るだけ」と決めておけば、OriginPolicy と `SameSite=Lax` が守ってくれます。

#### エラー3（エラーが出ない間違い）: エラーの画面で、何でもつかまえてしまった

`error_page` の最後の `return Err(error);` を、「何でも 500 の画面にする」に変えた場合です（`errs/e10_catchall.rs`）。

```text
@@OUT:u10_err_catchall@@
```

- **何が起きたか:** `/old` が、307 で移らずに、500 の「時間をおいてもう一度お試しください」になりました。リダイレクトも `Err` で届くので、何でもつかまえると、**移り先の案内まで、エラーの画面に変わってしまう**のです。ログイン画面へ移す 303（RustNote の `require_user`）も、同じく壊れます。
- **直し方:** 自分の画面にするのは、**知っている種類だけ**にして、ほかは `Err(error)` で返します（例10-1）。500 の画面を出したい種類（問1の `std::io::Error` など）は、名前を挙げて足します。

### RustNote ではここで使う

RustNote のレイアウト（`src/routes.rs`）は、例10-1と同じく、`not_found!()` と `error_boundary` を使っています（抜き出しています）。

```rust
// どのルートにも当たらない URL を NotFoundError にして、レイアウトの
// error_boundary（404 の画面）まで届ける
not_found!();
```

```rust
                <main class="container">
                    // ページで起きたエラーを、ステータスコード付きのエラー画面に変える
                    error_boundary(fallback: |error| error_page(cx, error), (slot))
                </main>
```

エラーの画面を作る `error_page`（`src/views.rs`）は、知っている種類を順に見分けます（抜き出しています）。

```rust
pub fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> {
    let (status, message) = if let Some(app_error) = error.downcast_ref::<AppError>() {
        log_if_internal(app_error.status_code(), &error);
        (app_error.status_code(), app_error.public_message())
```

- 自分のエラーの型（`AppError`・`RepoError`）と、Topcoat の `NotFoundError`・`BadRequestError` だけを画面にし、ほかは `return Err(error);` で返します（例10-1・エラー3）。`AppError::Internal`（DB に接続できない など）は 500 の画面にし、`log_if_internal` で**窓に記録**してから、利用者には「時間をおいて…」だけを見せます（問1と同じ考え方です。第4巻 UNIT 05）。
- お知らせ（「ノートを作成しました」）は、例10-2のテーマと同じ、ふつうのクッキー（`flash`）で、移った先のページに受け渡しています（`src/flash.rs`。第10巻 UNIT 02）。
- ログインは、例10-2と同じく `session::start`・`token_hash`・`stop` を使い、記録は DB の `sessions` 表に、ハッシュと期限を保存しています（`src/auth.rs`。第8巻 UNIT 05（準備中））。

### 練習問題

**問1:** 例10-1の `/boom` は、窓に何も出ないまま 500 になりました。ファイルの読み込みのような `std::io::Error` を、窓に記録してから、自分の 500 の画面にしてください。`/old` の 307 は、壊さないようにします。

**問2:** 例10-2のテーマのクッキーは、利用者が `theme=dark` と自分で書き換えられます。署名付きのクッキー（`signed_cookies`）にして、書き換えられたクッキーを読まないようにしてください。

**問3:** CSRF を防ぐために、ほかに何を組み合わせているでしょうか。例10-2の `Set-Cookie` と、エラー2を見て考えてください。

### 解答と解説

**問1の解答:**

`error_page` に、`std::io::Error` の場合を足します（`src/bin/u10_q1.rs`。足した所だけを抜き出しています）。

```rust
    } else if let Some(io) = error.downcast_ref::<std::io::Error>() {
        // 中のこと（ファイルの名前など）は、画面に出さずに窓へ書く
        eprintln!("[error] {io}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "時間をおいてもう一度お試しください".to_string(),
        )
    } else {
        return Err(error);
    };
```

```text
@@OUT:u10_q1@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u10_q1_server@@
```

- `/boom` は、レイアウト付きの 500 の画面になり、窓には `[error] 指定されたファイルが見つかりません。 (os error 2)` が記録されました。`/old` の 307 は、そのままです。
- 画面には「時間をおいて…」だけを出し、ファイルの名前やエラーの中身は出しません。中のことを見せると、攻撃する人の手がかりになるからです。

**問2の解答:**

鍵（`Key`）を `.app_context(Key::generate())` で登録し、`cookies(cx)` を `signed_cookies(cx)` に変えます（`src/bin/u10_q2.rs`）。

```text
@@OUT:u10_q2@@
```

- 署名付きのクッキーは、`theme=` のあとに、署名（Base64 の 44 文字）と値（`dark`）が並んだ形になりました。値は読めますが、書き換えると、署名が合わなくなります。
- 正しいクッキーを送ると dark、署名の無い `theme=dark` を送ると **light**（読まなかった）でした。`signed_cookies(cx).get(…)` は、署名が合わなければ `None` を返すからです。
- 鍵は、起動するたびに `Key::generate()` で作っているので、再起動すると前のクッキーは読めなくなります。本物のアプリでは、鍵を秘密の値として保存し（第12巻 UNIT 05（準備中））、起動のときに読み込みます。中身を見られたくもないときは、暗号化する `private_cookies` を使います。

**問3の解答:**

- **SameSite=Lax:** セッションのクッキーには `SameSite=Lax` が付いています。ブラウザは、ほかのサイトからの **POST** に、このクッキーを付けません。OriginPolicy が見落とした要求があっても、ログイン中の人として扱われません。
- **状態を変える操作は POST だけ:** エラー2のとおり、GET には `SameSite=Lax` のクッキーが付いてしまい、OriginPolicy も止めません。GET で状態を変えないことで、2つの守りが働く範囲に、すべての操作を入れています。
- **OriginPolicy:** ほかのサイトからの POST を、ルーターの最初で 403 にします。この3つを重ねて、CSRF を防いでいます。RustNote は、3つとも満たしています（第8巻 UNIT 06（準備中））。

### まとめ

- ページの `Err` は、レイアウトの `error_boundary` に届く。`not_found!()` で当たらない URL も届け、知っている種類（404・400）だけを `(status)` 付きの自分の画面にし、リダイレクトなどは `Err(error)` で Topcoat に返す。ルーターのエラーではないものは 500 になるので、記録してから画面にする。
- `cookies(cx)` でクッキーを読み書きし、大事な値は `signed_cookies`（署名）や `private_cookies`（暗号化）にする。セッションは `session::start` で発行し、ハッシュで記録して照合し、`stop` でクッキーと記録の両方を消す。
- CSRF は、OriginPolicy（ほかのサイトからの POST を 403）・`SameSite=Lax`・「状態を変える操作は POST だけ」を重ねて防ぐ。状態を変える操作を GET にすると、守りが効かない。
