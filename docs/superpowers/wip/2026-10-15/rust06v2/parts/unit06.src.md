## UNIT 06: フォーム・JSON・PRG

### この単元のゴール

- HTML のフォームから送られてきた値を、`Form<T>` で構造体として受け取り、成功したら `see_other` で 303 に移り（PRG）、失敗したら 400 で入力を残して出し直せる
- JSON を受け取る・返す API を、`#[route(POST)]` と `Json<T>` で作り、`(StatusCode::CREATED, Json(…))` のように状態コードを付けて返せる
- 形の違う本文・足りない項目・大きすぎる本文が、ページの関数より**前に** 400・413 で断られることを確かめられる
- `Deserialize` の付け忘れ・本文の引数を2つ書いたとき・303 を `Ok` で返したときのエラーを読める

### たとえ話でつかむ

役所の窓口に届く、2種類の書類を考えます。

- 1つは、**窓口で書く申込用紙**です。決まった欄（「氏名」「住所」）があり、人が手で書いて出します。これが HTML のフォームです。
- もう1つは、**ほかの役所から届く、決まった形式の電子データ**です。機械が読み書きするので、項目の名前も形も、きっちり決まっています。これが JSON です。
- 窓口の人は、書類を**受け付ける前に**、形を確かめます。欄が足りない、形式が違う、厚すぎて受け取れない、という書類は、担当者に回さずに、その場で返します。
- 申込用紙を受け付けたら、「控えは3番の窓口で」と案内して、別の窓口に移ってもらいます（UNIT 02 の PRG）。電子データには、「受け付けました。番号は 1 番です」と、**データで**返事をします。

| 役所の窓口 | Topcoat |
|---|---|
| 窓口で書く申込用紙 | HTML のフォーム（`Content-Type: application/x-www-form-urlencoded`） |
| 決まった形式の電子データ | JSON（`Content-Type: application/json`） |
| 受け付ける前に形を確かめる | `Form<T>`・`Json<T>`（形が違えば 400、大きすぎれば 413） |
| 「控えは3番の窓口で」 | `see_other(…)`（303 で GET の画面へ移す。PRG） |
| 「受け付けました。番号は 1 番」 | `(StatusCode::CREATED, Json(memo))`（201 と JSON の返事） |

> **用語:** JSON（ジェイソン：`{"title":"milk"}` のように、名前と値の組を `{}` で、並びを `[]` で書く、データの書き方。プログラムどうしのやり取りに使う）

<!-- -->

> **用語:** API（エーピーアイ：プログラムから使うための窓口。ここでは、HTML の画面ではなく、JSON を返す URL のこと）

<!-- -->

> **用語:** 201 Created（「新しく作りました」という返事。作ったものを本文で返すことが多い）

### 図で理解する

本文を受け取ってから、関数が動くまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="170" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="105" y="54" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">POST の本文</text>
<text x="105" y="76" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">title=milk</text>
<text x="105" y="96" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">{"title":"milk"}</text>
<rect x="240" y="20" width="220" height="100" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="350" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">Form&lt;MemoForm&gt; / Json&lt;NewMemo&gt;</text>
<text x="350" y="66" font-size="11" fill="#0f172a" text-anchor="middle">① Content-Type を確かめる</text>
<text x="350" y="86" font-size="11" fill="#0f172a" text-anchor="middle">② 大きさを確かめる（既定 2 MiB まで）</text>
<text x="350" y="106" font-size="11" fill="#0f172a" text-anchor="middle">③ 構造体に変える（serde）</text>
<rect x="510" y="30" width="190" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="56" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">関数が動く</text>
<text x="605" y="78" font-size="11" fill="#0f172a" text-anchor="middle">input.title を使って</text>
<text x="605" y="96" font-size="11" fill="#0f172a" text-anchor="middle">中身を確かめる・保存する</text>
<line x1="190" y1="70" x2="236" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<line x1="460" y1="70" x2="506" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<rect x="240" y="160" width="220" height="80" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="350" y="184" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">関数は動かない</text>
<text x="350" y="206" font-size="11" fill="#0f172a" text-anchor="middle">形が違う・項目が無い → 400</text>
<text x="350" y="226" font-size="11" fill="#0f172a" text-anchor="middle">大きすぎる → 413</text>
<line x1="350" y1="120" x2="350" y2="156" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<text x="560" y="180" font-size="11" fill="#475569" text-anchor="middle">形は合っていても、中身の決まり</text>
<text x="560" y="198" font-size="11" fill="#475569" text-anchor="middle">（空のタイトルなど）は、</text>
<text x="560" y="216" font-size="11" fill="#475569" text-anchor="middle">関数の中で確かめる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 本文の受け取り。Form・Json は、関数が動く前に、種類・大きさ・形を確かめ、合わなければ 400 や 413 で断る。空のタイトルのような中身の決まりは、関数の中で確かめる</div>
</div>

- `Form<T>`・`Json<T>` は、関数の**引数**に書きます。Topcoat は、関数を呼ぶ前に本文を読み、構造体 `T` に変えてから渡します。変えられなければ、関数は呼ばれません。
- 本文の「形」（項目があるか、数か文字か）は `Form`・`Json` が、本文の「中身の決まり」（空でないか、80 文字までか）は関数が確かめます。2段に分けて確かめるのは、第10巻 UNIT 02 の `NoteForm` と `NewNote::parse` と同じです。

画面とデータで、返し方が違います。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="330" height="165" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">人が見る画面（#[page(POST)]）</text>
<text x="40" y="72" font-size="12" fill="#0f172a">成功: Err(see_other(URL).into())</text>
<text x="56" y="92" font-size="11" fill="#475569">→ 303・location。ブラウザが GET で開き直す</text>
<text x="40" y="122" font-size="12" fill="#0f172a">失敗: Ok(view! { (StatusCode::BAD_REQUEST) … })</text>
<text x="56" y="142" font-size="11" fill="#475569">→ 400 と、入力を残したフォーム（HTML）</text>
<text x="40" y="170" font-size="11" fill="#475569">返すのは、いつも HTML か、移り先</text>
<rect x="370" y="20" width="330" height="165" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="535" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">プログラムが読む API（#[route(POST)]）</text>
<text x="390" y="72" font-size="12" fill="#0f172a">成功: Ok((StatusCode::CREATED, Json(memo)))</text>
<text x="406" y="92" font-size="11" fill="#475569">→ 201 と、作ったものの JSON</text>
<text x="390" y="122" font-size="12" fill="#0f172a">一覧: Ok(Json(memos))</text>
<text x="406" y="142" font-size="11" fill="#475569">→ 200 と、JSON の配列</text>
<text x="390" y="170" font-size="11" fill="#475569">移らずに、結果をデータで返す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 返し方の違い。画面は、成功したら 303 で GET の画面へ移し（PRG）、失敗したら 400 で出し直す。API は、状態コードと JSON をそのまま返す</div>
</div>

### まず動かす最小コード

#### 例6-1: フォームを受け取り、303 で移る

> **▼ このコードがやること（先に日本語で）:** メモの一覧と、タイトルを書くフォームの画面を、部品 `memo_page` で作ります。`GET /` はその画面を返します。`POST /` は、フォームの `title` を `Form<MemoForm>` で受け取り、前後の空白を取って空なら 400 で出し直し、空でなければメモに足して、303 で `GET /` へ移ります。メモは、プログラムが動いている間だけ、`static` の `Mutex<Vec<String>>` に覚えておきます（本物のアプリでは DB に保存します）。

```rust
@@FILE:web_lab/src/bin/u06_form.rs@@
```

この例から、構造体を JSON やフォームと変換する `serde` を使うので、`web_lab` に足します。

```powershell
cargo add serde@=1.0.229 --features derive
cargo run --bin u06_form
```

2つ目の窓から、フォームと同じ形の本文を、`-d` で POST します（`%E8%B2%B7%E3%81%84%E7%89%A9` は「買い物」）。

```powershell
curl.exe -i -d "title=%E8%B2%B7%E3%81%84%E7%89%A9" http://127.0.0.1:3000/
curl.exe -s http://127.0.0.1:3000/
```

**▼ 実行結果**（date と vary の行は省いています）

```text
@@OUT:u06_form_ok@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Form<T> は、本文を構造体にして渡す

- `curl.exe -d "title=…"` は、ブラウザのフォームと同じ形（`Content-Type: application/x-www-form-urlencoded`）で本文を送ります。`create` の引数の `Form(input): Form<MemoForm>` が、本文の `title=…` を `MemoForm { title: "買い物" }` に変えて渡しました。
- `MemoForm` に付けた `#[derive(Deserialize)]` が、「本文の `title` を、構造体の `title` に入れる」変換を作ります（エラー1）。HTML の `<input name="title">` の `name` と、構造体の項目名をそろえます（第10巻 UNIT 02 のエラー2）。

##### 解説2: 成功したら、Err(see_other(…)) で 303

- 成功したときの返事は `303 See Other` と `location: /` だけで、本文はありません。そのあとの `GET /` で、「買い物」が一覧に入っていました。
- `see_other(…)` は、Topcoat の `router::error` にある**エラーの形の返事**です。`Err(… .into())` で返すと、画面を作るのをやめて、303 の返事になります。画面（`impl View`）を返す関数の中で、「画面ではなく移り先を返す」ために、エラーの道を使うのです（エラー3）。
- 移り先の URL は、`href!(home).resolve(cx)` で、ページの関数の名前から作っています（UNIT 03）。

##### 解説3: 同じ URL の GET と POST

- `home` は `#[page]`（GET）、`create` は `#[page(POST)]` です。同じモジュールなので、どちらも `/` ですが、メソッドが違うので取り合いません（UNIT 03 のエラー1）。フォームの `action="/"` と `method="post"` が、`create` に届きます。

#### ▼ 失敗したとき（空白だけのタイトル）

```powershell
curl.exe -i -d "title=++" http://127.0.0.1:3000/
```

```text
@@OUT:u06_form_bad@@
```

- `++` は、フォームの書き方で空白2つです。前後の空白を取ると空なので、`400 Bad Request` と、エラーの文の付いたフォームが返りました。入力欄の `value="  "` は、送られたまま残しています。
- 400 のときは、`view!` の最初に `(StatusCode::BAD_REQUEST)` を置いて、状態コードを決めています（UNIT 05）。ここでは移らずに、同じ画面を返します。

### もう一歩: 実務に近い形

#### 例6-2: JSON を受け取り、JSON で返す API

人ではなくプログラムが使う窓口（API）を作ります。

> **▼ このコードがやること（先に日本語で）:** `GET /api/memos` で、保存したメモを JSON の配列で返します。`POST /api/memos` で、`{"title":"…"}` の JSON を `Json<NewMemo>` で受け取り、番号を付けてメモに足し、`201 Created` と、足したメモの JSON を返します。

```rust
@@FILE:web_lab/src/bin/u06_api.rs@@
```

PowerShell 5.1 では、`"` を含む JSON をコマンドに直接書くと、うまく渡らないことがあります。JSON はファイルに書いて、`--data-binary "@ファイル名"` で送ります（英数字だけにして `-Encoding ascii` で書くと、文字化けもしません）。

```powershell
cargo run --bin u06_api
```

```powershell
Set-Content memo.json '{"title":"milk"}' -Encoding ascii
curl.exe -i http://127.0.0.1:3000/api/memos
curl.exe -i -H "Content-Type: application/json" --data-binary "@memo.json" http://127.0.0.1:3000/api/memos
curl.exe -i http://127.0.0.1:3000/api/memos
```

**▼ 実行結果**

```text
@@OUT:u06_api_ok@@
```

- 最初の `GET` は `[]`（空の配列）でした。`Json(…)` で包んで返すと、Topcoat が構造体を JSON に変え、`content-type: application/json` を付けます。`Memo` に付けた `#[derive(Serialize)]` が、構造体から JSON への変換を作ります。
- `POST` は `201 Created` で、本文は `{"id":1,"title":"milk"}` でした。`Ok((StatusCode::CREATED, Json(memo)))` のように、**組（タプル）の最初に状態コード**を置くと、その状態コードで返ります（RustNote の `/health` も同じ形です）。
- `#[route(POST)]` は、画面ではなく、値をそのまま返事にします。レイアウトにも包まれません（UNIT 03）。API では、303 で移らずに、結果をデータで返します。

形の違う JSON を送ってみます（`empty.json` は `{}`、`broken.json` は `{"title":` で終わっているファイルです）。

```text
@@OUT:u06_api_bad@@
```

- `{}` は、``missing field `title` ``（`title` の項目がありません）の 400 です。`broken.json` は、`EOF while parsing a value`（値を読んでいる途中で終わりました）の 400 です。どちらも、**関数が動く前に**、`Json<NewMemo>` が断りました。メッセージには、何行目の何文字目か（`line 1 column 2`）まで出ます。
- この 400 の本文は、送った人（プログラムを書いた人）に向けたものです。人が見る画面では、こうした英語のメッセージをそのまま出さずに、RustNote のように、日本語の画面に変えて出します（UNIT 10）。

### よくあるエラーと読み方

#### エラー1: フォームの構造体に #[derive(Deserialize)] を付け忘れた（E0277）

```rust
@@FILE:errs/e06_noderive.rs@@
```

```text
@@OUTHEAD:u06_err_noderive:25@@
```

（このあとに、同じ所を指す `E0277` がもう1つ続き、その中に ``note: for local types consider adding `#[derive(serde::Deserialize)]` to your `MemoForm` type`` という案内があります）

- **何が起きたか:** 「`MemoForm` に、`DeserializeOwned` という trait が実装されていません」というエラーです。最後の `note:` の ``required for `Form<MemoForm>` to implement `FromRequest` `` は、「`Form<MemoForm>` が本文を読めるためには、`MemoForm` が本文から作れる（`Deserialize`）必要があります」という意味です。
- **直し方:** 2つ目のエラーの `note:` のとおり、`#[derive(serde::Deserialize)]`（`use serde::Deserialize;` してあれば `#[derive(Deserialize)]`）を、`MemoForm` に付けます。JSON で返す構造体には、`Serialize` を付けます。

#### エラー2: 本文の引数を2つ書いた

フォームと JSON の両方を受け取ろうとした場合です（`errs/e06_twobody.rs`。抜き出しています）。

```rust
#[page(POST)]
async fn create(Form(form): Form<MemoForm>, Json(json): Json<MemoForm>) -> Result<impl View> {
```

```text
@@OUT:u06_err_twobody@@
```

- **何が起きたか:** 「ページの関数は、本文の引数を2つ以上受け取れません」というエラーです。本文は、届いたものを**1回しか読めない**ので、`Form` と `Json` の両方で読むことはできません。そのあとの `unused imports` の警告は、エラーで関数が作られなかったために出たもので、気にしなくて大丈夫です。
- **直し方:** 1つの URL には、1つの形の本文だけを受け取ります。フォームと JSON の両方を受け付けたいなら、URL を分けます（例6-1の `/` と、例6-2の `/api/memos` のように）。`cx: &Cx` は本文ではないので、本文の引数と一緒に書けます。

#### エラー3: 303 で移るつもりで、Ok(see_other(…)) と書いた（E0308）

```rust
@@FILE:errs/e06_okredirect.rs@@
```

```text
@@OUT:u06_err_okredirect@@
```

- **何が起きたか:** 「型が合いません」というエラーです。この関数の戻り値は `Result<impl View>` で、23 行目で `Ok(view! { … })` を返しているので、`Ok` の中身は「その `view!` の型」に決まりました。25 行目の `Ok(see_other(…))` の中身は `SeeOther` なので、合わないと言われました。
- **直し方:** `Err(see_other(…).into())` と、**エラーの道**で返します（例6-1）。`#[route(POST)]` で、戻り値を `Result<SeeOther>` と書いた関数なら、`Ok(see_other(…))` で返せます。ページ（`#[page]`）は画面を返すものなので、移り先は `Err` で返す、と覚えておきます。

### RustNote ではここで使う

ノートの作成（`src/routes/notes.rs` の `create`）は、例6-1と同じ形です（第10巻 UNIT 02 でくわしく見ました）。

```rust
#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;
```

- `Form<NoteForm>` で受け取り、`NewNote::parse` で中身の決まりを確かめ、成功なら `Err(see_other(…).into())` で 303、失敗なら `(StatusCode::BAD_REQUEST)` で 400 です。

ログアウト（`src/routes/logout.rs`）は、本文を受け取らない POST です（抜き出しています）。

```rust
// レイアウト（エラー画面）を通すため #[page(POST)] にしている。成功すると必ず 303 なので、
// 画面として返す中身は無い（`()`）。
#[page(POST)]
pub async fn logout(cx: &Cx) -> Result<()> {
    log_out(cx).await?;
    flash::set(cx, Flash::LoggedOut);
    Err(see_other(href!(crate::routes::login::login_page).resolve(cx)).into())
}
```

- 状態を変える操作（ログアウト）なので、GET ではなく POST にしています。戻り値は `Result<()>` で、成功しても `Ok` は返さず、いつも 303 です。
- 失敗したとき（DB に接続できないなど）に、RustNote のエラー画面（レイアウトの `error_boundary`）を通したいので、`#[route]` ではなく `#[page(POST)]` にしています。

ヘルスチェック（`src/routes/health.rs`）は、例6-2と同じく、状態コードと JSON の組を返す API です。

```rust
#[route(GET)]
pub async fn health(cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
```

- DB に問い合わせて、成功なら `(StatusCode::OK, Json(Health { status: "ok" }))`、失敗なら `(StatusCode::SERVICE_UNAVAILABLE, …)` を返します。人ではなく、Fly.io が読む窓口です（第12巻 UNIT 06（準備中））。

### 練習問題

**問1:** `curl.exe -i -L -d "title=%E6%9C%AC" http://127.0.0.1:3000/`（`-L` は、移り先へ自動で移る指定）で例6-1にメモを送ると、何回の返事が表示されるでしょうか。2回目の要求は、GET と POST のどちらでしょうか。

**問2:** 例6-2の `/api/memos` に、`curl.exe -i -d "title=milk"`（フォームの形）で送ると、どうなるでしょうか。

**問3:** 3 MiB の JSON（`{"title":"aaa…"}` の `a` を 3,145,728 個）を `big.json` に作り、例6-2の `/api/memos` に送ると、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
@@OUT:u06_q1@@
```

- 2回の返事が表示されました。1回目は POST の返事（303）、2回目は、curl が `location: /` へ移ったあとの返事（200 と、「本」が増えた一覧）です。
- 2回目は **GET** です。303 は「次は GET で取りに来て」という決まりなので、curl もブラウザも、本文を送り直さずに GET で開きます。だから、再読み込みしても、メモは二重になりません（第10巻 UNIT 02 の PRG）。

**問2の解答:**

```text
@@OUT:u06_q2@@
```

- 400 で、`expected request with Content-Type: application/json`（`Content-Type: application/json` の要求が来るはずでした）でした。`-d` で送ると、curl は `Content-Type: application/x-www-form-urlencoded` を付けるので、`Json<NewMemo>` が、本文を読む前に断りました。
- JSON の API に送るときは、`-H "Content-Type: application/json"` を付けます。ブラウザの `fetch` などで送るときも同じです。

**問3の解答:**

PowerShell では、次のように作れます。

```powershell
Set-Content big.json ('{"title":"' + ('a' * 3145728) + '"}') -Encoding ascii
curl.exe -i -H "Content-Type: application/json" --data-binary "@big.json" http://127.0.0.1:3000/api/memos
```

```text
@@OUT:u06_q3@@
```

- `413 Payload Too Large` で、本文は `content too large` でした。`Json` や `Form` は、本文を全部読んでから変えるので、**既定で 2 MiB まで**しか受け付けません。とても大きな本文を送りつけて、サーバーのメモリを使い切らせる攻撃を防ぐためです。
- 最初の `HTTP/1.1 100 Continue` は、大きな本文を送る前に、curl が「送ってよいですか」と尋ね（`Expect: 100-continue`）、サーバーが「どうぞ」と答えた返事です。そのあと、大きすぎると分かった時点で 413 が返りました。
- もっと大きな本文（ファイルのアップロードなど）を受け付けたい URL だけ、`BodyLimit::max(…).at("/upload")` で上限を変えられます。RustNote は、ノートの本文が最大 10,000 文字なので、2 MiB のままで足ります。

### まとめ

- フォームは `Form<T>`、JSON は `Json<T>` で、本文を構造体（`#[derive(Deserialize)]`）として受け取る。種類・大きさ（既定 2 MiB）・形が合わなければ、関数が動く前に 400・413 で断られる。中身の決まりは関数の中で確かめる。
- 画面（`#[page(POST)]`）は、成功したら `Err(see_other(…).into())` で 303 に移り（PRG）、失敗したら `(StatusCode::BAD_REQUEST)` で 400 の画面を返す。
- API（`#[route(POST)]`）は、`Ok((StatusCode::CREATED, Json(…)))` のように、状態コードと JSON をそのまま返す。本文の引数は1つだけ。303 を `Ok` で返すと型が合わない。
