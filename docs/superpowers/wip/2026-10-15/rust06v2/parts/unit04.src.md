## UNIT 04: URL の部品（path・query）と Cx

### この単元のゴール

- `/notes/7` の `7` のような**パスの部品**を、`module_param!` と `path_param::<Id>(cx)` で、型の付いた値として読める
- `?q=rust&page=2` のような**クエリ**を、`#[query_params]` を付けた構造体と `query_params::<T>(cx)` で読み、無い・空のときに `None` になることを説明できる
- 読めなかったときの返事を `error = not_found`（404）・`error = bad_request`（400）で選び、自分で作った型（1 以上の数だけの `NoteId`）で読める
- `cx: &Cx` から、メソッド・URL・ヘッダーを読む関数を書き、型の取り違え（E0308）・`Option` の付け忘れ・`view!` に入れられない型（E0277）を読める

### たとえ話でつかむ

宅配便の伝票を考えます。

- 伝票には、**住所**を書く欄があります。「東京都 / 港区 / 7番地」のように、上から順に、決まった並びで書きます。この並びが違えば、別の家に届きます。
- 住所とは別に、**備考欄**があります。「午前中に届けて」「置き配でよい」のように、書いても書かなくてもよいことを、名前と中身の組で書きます。
- 受付の人は、伝票を読むとき、「7番地」の「7」を**数として**読みます。「七」や「ABC」と書いてあったら、「そんな番地はありません」と返します。
- 備考欄の「午前中」が空欄なら、「指定なし」として扱います。
- 受付の人の机には、**伝票そのもの**が置いてあります。配達の係も、仕分けの係も、その伝票を見に来れば、必要なことが読めます。

| 宅配便 | Topcoat の要求 |
|---|---|
| 住所の欄（決まった並び） | URL の**パス**（`/notes/7`）。`module_param!` で、区切りを1つ `{id}` にする |
| 「7番地」を数として読む | `path_param::<Id>(cx)`（`u64` や `NoteId` に変える） |
| 「そんな番地はありません」 | `error = not_found`（読めなければ 404） |
| 備考欄（名前と中身の組） | URL の**クエリ**（`?q=rust&page=2`）。`#[query_params]` の構造体で読む |
| 空欄なら「指定なし」 | `Option<T>` の項目。無い・空なら `None` |
| 机の上の伝票 | `cx: &Cx`（今の要求の中身。どの関数からでも読める） |

> **用語:** パスパラメーター（path parameter：URL のパスの中で、変わる部分。`/notes/7` の `7`。Topcoat では `{id}` のように書く）

<!-- -->

> **用語:** クエリ文字列（query string：URL の `?` より後ろの、`名前=中身` を `&` でつないだ部分。検索の言葉やページの番号に使う）

<!-- -->

> **用語:** Cx（コンテキスト：今の要求の中身と、アプリで共有する値をまとめて持っている入れ物。`cx: &Cx` を引数にすると、Topcoat が渡してくれる）

### 図で理解する

URL のどの部分を、どう読むかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="680" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="40" y="48" font-size="14" fill="#475569" font-family="Consolas, monospace">http://127.0.0.1:3000</text>
<text x="246" y="48" font-size="14" fill="#1e40af" font-weight="700" font-family="Consolas, monospace">/notes/7</text>
<text x="330" y="48" font-size="14" fill="#c2410c" font-weight="700" font-family="Consolas, monospace">?q=rust&amp;page=2</text>
<rect x="40" y="100" width="300" height="150" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="190" y="124" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">パス（住所）</text>
<text x="60" y="150" font-size="12" fill="#0f172a" font-family="Consolas, monospace">mod notes { mod id {</text>
<text x="60" y="170" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  module_param!(id: u64,</text>
<text x="60" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">    error = not_found);</text>
<text x="60" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">path_param::&lt;Id&gt;(cx)?</text>
<text x="190" y="238" font-size="11" fill="#1e40af" text-anchor="middle">→ &amp;u64 の 7（読めなければ 404）</text>
<rect x="380" y="100" width="300" height="150" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="530" y="124" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">クエリ（備考欄）</text>
<text x="400" y="150" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[query_params(error = bad_request)]</text>
<text x="400" y="170" font-size="12" fill="#0f172a" font-family="Consolas, monospace">struct NotesQuery {</text>
<text x="400" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  q: Option&lt;String&gt;, page: Option&lt;u32&gt; }</text>
<text x="400" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">query_params::&lt;NotesQuery&gt;(cx)?</text>
<text x="530" y="238" font-size="11" fill="#c2410c" text-anchor="middle">→ q = Some("rust")・page = Some(2)</text>
<line x1="280" y1="64" x2="200" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0401)"/>
<line x1="420" y1="64" x2="510" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0401)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: URL の部品の読み方。パスの区切りは module_param! で型を決めて path_param で、? より後ろは #[query_params] の構造体と query_params で読む</div>
</div>

- パスの部品は、**モジュールの並び**で場所が決まります。`mod notes` の中の `mod id` に `module_param!(id: u64, …)` を書くと、そのモジュールの区切りが `{id}` になり、URL は `/notes/{id}` になります（UNIT 03 のモジュールルーティングの続きです）。
- クエリは、URL の場所には関係ありません。構造体の項目の名前（`q`・`page`）が、`?` より後ろの名前と合うものを読みます。

読めたとき・読めなかったときの返事です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="120" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">URL</text>
<text x="360" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">読んだ結果</text>
<text x="600" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">返事</text>
<rect x="20" y="36" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="58" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes/7</text>
<rect x="260" y="36" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="58" font-size="12" fill="#047857" text-anchor="middle">Ok(&amp;7)</text>
<rect x="500" y="36" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="58" font-size="12" fill="#047857" text-anchor="middle">ページを動かす → 200</text>
<rect x="20" y="82" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="104" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes/abc</text>
<rect x="260" y="82" width="200" height="34" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="360" y="104" font-size="12" fill="#b91c1c" text-anchor="middle">Err（数ではない）</text>
<rect x="500" y="82" width="200" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="104" font-size="12" fill="#c2410c" text-anchor="middle">error = not_found → 404</text>
<rect x="20" y="128" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="150" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes?page=abc</text>
<rect x="260" y="128" width="200" height="34" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="360" y="150" font-size="12" fill="#b91c1c" text-anchor="middle">Err（page が数ではない）</text>
<rect x="500" y="128" width="200" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="150" font-size="12" fill="#c2410c" text-anchor="middle">error = bad_request → 400</text>
<rect x="20" y="174" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="196" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes?page=</text>
<rect x="260" y="174" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="196" font-size="12" fill="#047857" text-anchor="middle">page = None（空は「無い」）</text>
<rect x="500" y="174" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="196" font-size="12" fill="#047857" text-anchor="middle">1 ページ目として 200</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 読んだ結果と返事。読めなかったときは、宣言に書いた error = … の返事になり、ページの関数は動かない。Option の項目は、無いときも空のときも None になる</div>
</div>

### まず動かす最小コード

#### 例4-1: パス・クエリ・要求の中身を読む

> **▼ このコードがやること（先に日本語で）:** 4つのページを作ります。`/notes` は、クエリの `q`（検索の言葉）と `page`（ページの番号）を読んで表示します。`/notes/{id}` は、パスの `{id}` を `u64` として読みます。`/users/{name}` は、パスの `{name}` を文字のまま読みます。`/whoami` は、`cx` から、メソッド・URL・`User-Agent` ヘッダー（どんなプログラムから来たか）を読んで表示します。

```rust
@@FILE:web_lab/src/bin/u04_params.rs@@
```

```powershell
cargo run --bin u04_params
```

2つ目の窓から、4つの URL を開きます（URL に `&` が入るので、`"…"` で囲みます。`%E5%A4%AA%E9%83%8E` は「太郎」を UTF-8 で書いたものです）。

```powershell
curl.exe -i "http://127.0.0.1:3000/notes"
curl.exe -i "http://127.0.0.1:3000/notes?q=rust&page=2"
curl.exe -i "http://127.0.0.1:3000/notes/7"
curl.exe -i "http://127.0.0.1:3000/users/%E5%A4%AA%E9%83%8E"
```

**▼ 実行結果**（date と vary の行は省いています）

```text
@@OUT:u04_params_curl@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: module_param! は、区切りを {id} にして、型を決める

- `mod notes` の中の `mod id` に `module_param!(id: u64, error = not_found)` と書きました。これで、`mod id` の区切りは `id` ではなく `{id}` になり、URL は `/notes/{id}` です。`/notes/7` の `7` が、`{id}` に当たります。
- `module_param!` は、`id` から `Id` という型も作ります（名前の頭を大文字にした形）。`path_param::<Id>(cx)` と書くと、「`Id` の部品を読んで」という意味になり、`u64` に変えた値（`&u64`）が返ります。
- `error = not_found` があるので、読めなかったとき、`path_param::<Id>(cx)?` の `?` で、404 の返事になります（例4-1の次の「読めなかったとき」で確かめます）。

##### 解説2: 型を書かなければ、文字のまま読める

- `/users/{name}` の `module_param!(name)` には、型を書いていません。このときは、区切りの文字が `&str` のまま返ります。数に変えないので、失敗することもありません（`error = …` も書けません）。
- `%E5%A4%AA%E9%83%8E` は、`%` の後ろの2文字ずつが1バイトを表す書き方（パーセントエンコーディング）で、「太郎」の UTF-8 の6バイトです。Topcoat は、これを元に戻してから渡すので、ページでは「太郎」として読めました。

##### 解説3: #[query_params] は、構造体に ? より後ろを読み込む

- `#[query_params(error = bad_request)]` を付けた `NotesQuery` は、`q` と `page` の2つの項目を持ちます。`query_params::<NotesQuery>(cx)?` で、`?q=rust&page=2` が `q = Some("rust")`・`page = Some(2)` になりました。
- `/notes`（クエリが無い）では、どちらも `None` で、`unwrap_or_else`・`unwrap_or(1)` の値（「（なし）」と 1）が使われました。**クエリは、いつも付いているとは限らない**ので、項目は `Option<T>` にします（エラー1）。
- クエリの構造体は、URL の場所とは関係ありません。`&Cx` を持っている関数なら、どこからでも同じ構造体を読めます。1つの要求の中では、最初の1回だけ読み込んで、あとは覚えておいた結果を返します。

##### 解説4: &Cx があれば、要求の中身を読める

- `request_summary(cx)` は、ページではない、ふつうの関数です。`cx: &Cx` を受け取れば、`method(cx)`（`GET` など）・`uri(cx)`（`/whoami` など）・`headers(cx)`（ヘッダーの一覧）を読めます。

```powershell
curl.exe -i -A "RustNote-test/1.0" http://127.0.0.1:3000/whoami
```

```text
@@OUT:u04_whoami@@
```

- `-A` で、curl が送る `User-Agent` を変えました。ページの関数ではなく、そこから呼んだ `request_summary` が、`cx` から読んでいます。**`cx` を渡せば、どの関数でも要求の中身を読める**ので、「要求の中身を1か所で用意して、全部の関数に配る」ような仕組み（ミドルウェア）を作らなくて済みます（UNIT 07）。

#### ▼ 読めなかったときの返事

```powershell
curl.exe -i "http://127.0.0.1:3000/notes?page=abc"
curl.exe -i "http://127.0.0.1:3000/notes?page="
curl.exe -i "http://127.0.0.1:3000/notes/abc"
```

```text
@@OUT:u04_bad_curl@@
```

- `?page=abc` は 400 で、本文は ``bad request: invalid query value: invalid digit found in string (at `page`)`` でした。「クエリの値が不正です。文字列に数字ではない文字があります（`page` の所で）」という意味で、**どの項目が読めなかったか**まで教えてくれます。
- `?page=`（空）は、エラーにならず、1 ページ目でした。ブラウザのフォームは、空の入力欄も `page=` として送ってくるので、Topcoat は「空」を `None`（無い）として扱います。
- `/notes/abc` は 404 です。`abc` は `u64` に変えられないので、`error = not_found` の返事になりました。ページの関数は動いていません。

### もう一歩: 実務に近い形

#### 例4-2: 自分で作った型で読む（1 以上の数だけの NoteId）

`u64` では、`/notes/0` も読めてしまいます。ノートの ID は 1 から始まるので、0 は「無いノート」です。**1 以上の数しか作れない型**を作り、その型で読みます。RustNote の `NoteId` と同じ考え方です（第4巻 UNIT 01）。

> **▼ このコードがやること（先に日本語で）:** `NoteId` という型を作り、文字から作る `FromStr`（数でなければ `NotANumber`、1 未満なら `NotPositive` のエラー）と、表示の `Display` を書きます。`module_param!(id: NoteId, error = not_found)` で、URL の `{id}` を `NoteId` として読みます。

```rust
@@FILE:web_lab/src/bin/u04_noteid.rs@@
```

```powershell
cargo run --bin u04_noteid
```

```text
@@OUT:u04_noteid_curl@@
```

- `/notes/7` は 200、`/notes/0`・`/notes/-5`・`/notes/abc` は、どれも 404 になりました。`module_param!` は、型に書いてある `FromStr` を使って文字を変えるので、**「1 以上」という決まりを、型に書いておくだけで**、URL の読み取りにも効きます。
- ページの関数の中では、`id` は必ず 1 以上の `NoteId` です。関数の中で「0 だったら…」と確かめる必要はありません。
- 画面に出すときは `id.to_string()` にしています。`view!` に `(id)` とそのまま入れるとエラーになるからです（エラー3）。

### よくあるエラーと読み方

#### エラー1: クエリの項目を Option にし忘れた

`page: Option<u32>` を、`page: u32` と書いてしまった場合です（`errs/e04_required.rs`）。コンパイルは通ります。

```rust
@@FILE:errs/e04_required.rs@@
```

```text
@@OUT:u04_err_required@@
```

- **何が起きたか:** `?page=2` を付けたときは読めましたが、付けないで `/notes` を開くと、400 になりました。``missing field `page` ``（`page` の項目がありません）です。`Option` ではない項目は、「必ず付いている」という決まりになるからです。
- **直し方:** 付いていなくてもよい項目は、`Option<u32>` にして、ページの関数で `unwrap_or(1)` のように決まった値を使います。
- **学べること:** 一覧のページは、ふつう、クエリを付けずに開きます（メニューのリンクなど）。クエリの項目は、ほぼ全部 `Option` にすると考えてかまいません。

#### エラー2: path_param の結果を、そのまま u64 として受け取った（E0308）

`error = …` を書かない `module_param!(id: u64)` で、結果を `u64` の変数に入れようとした場合です（`errs/e04_type.rs`。抜き出しています）。

```rust
        module_param!(id: u64); // error = ... を書いていない

        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id: u64 = path_param::<Id>(cx);
```

```text
@@OUT:u04_err_type@@
```

- **何が起きたか:** 「型が合いません。`u64` のはずが、`Result<&u64, &ParseIntError>` です」というエラーです。`path_param` は、**読めなかったかもしれない**ので、`Result` を返します。中身は `u64` そのものではなく、借りた `&u64` です。
- **直し方:** 読めなかったときの返事を宣言で決める（`module_param!(id: u64, error = not_found)` にして `path_param::<Id>(cx)?`）か、呼んだ所で決めます（`path_param::<Id>(cx).ok_or_not_found()?`。`RouterErrorExt` を `use` します）。値が `u64` で欲しいときは、`*` を付けて `let id: u64 = *path_param::<Id>(cx)?;` と書きます（RustNote の `show` も `*` を付けています）。

#### エラー3: 自分で作った型を、view! にそのまま入れた（E0277）

例4-2で、`(id.to_string())` ではなく、`(id)` と書いた場合です（`errs/e04_display.rs`）。

```rust
            Ok(view! { <h1>"ノート " (id) " 番"</h1> })
```

```text
@@OUTHEAD:u04_err_display:22@@
```

（このあとに、同じ形の `E0277` がもう1つ続きます）

- **何が起きたか:** 「`&NoteId` に、`NodeViewParts` という trait が実装されていません」というエラーです。`view!` の `( )` に入れられるのは、文字（`&str`・`String`）・数（`i32`・`u64` など）・`bool` など、Topcoat が**画面に出し方を知っている型**だけです（`help:` に、入れられる型の一部が並んでいます）。`NoteId` に `Display` を書いても、それだけでは入れられません。
- **直し方:** `(id.to_string())` で文字にしてから入れます。数を入れたいなら `(id.get())` のように、中の数を取り出してもかまいません。
- **学べること:** 「どう表示するか」を `view!` が勝手に決めないので、HTML に入る文字は、いつも自分で選んだものになります。エスケープ（危ない文字の書き換え）の話は UNIT 05で扱います。

### RustNote ではここで使う

ノートの一覧（`src/routes/notes.rs`）は、クエリを次の構造体で読んでいます（抜き出しています）。

```rust
/// URL のクエリ（?q=...&page=...）。page が数字でなければ 400
#[query_params(error = bad_request)]
pub struct NotesParams {
    q: Option<String>,
    page: Option<u32>,
}

#[page]
pub async fn index(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let params = query_params::<NotesParams>(cx)?;
    let query = NoteQuery::new(params.q.as_deref(), params.page);
```

- 例4-1の `NotesQuery` と同じ形です。読んだ値を、そのまま SQL に使うのではなく、`NoteQuery::new` で、「前後の空白を取る」「空白だけなら検索しない」「ページは 1 以上」と整えてから使います（第10巻 UNIT 01）。
- 最初に `require_user(cx)` でログインを確かめてから、クエリを読んでいます。どちらも `cx` から読むので、順番は自分で決められます。

ノートの詳細（`src/routes/notes/id.rs`）は、`{id}` を `NoteId` で読んでいます（抜き出しています）。

```rust
// URL の {id} を NoteId に変換する。`Id` という型ができ、path_param::<Id>(cx) で読める。
// 変換に失敗したら NotFoundError（404）になる。
module_param!(pub id: NoteId, error = not_found);

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
```

- 例4-2と同じく、`NoteId` の `FromStr`（`src/domain.rs`）が、数でない・1 未満の ID を断ります。`pub` が付いているのは、ほかのファイル（編集・完了・削除のページ）からも `Id` を使うからです。`/notes/{id}/edit` は `notes/id/edit.rs` で、親のモジュールの `Id` を読みます。
- 他人のノートの ID を開いても、無い ID と同じ 404 にしています（第8巻 UNIT 01・第10巻 UNIT 03（準備中））。

### 練習問題

**問1:** 例4-1のアプリで、`curl.exe -i "http://127.0.0.1:3000/notes?q=a+b%26c"` を開くと、「検索:」のあとに何が出るでしょうか。`+` と `%26` は、それぞれ何に変わるでしょうか。

**問2:** 例4-1のアプリで、`/users/a%2Fb`・`/users/a/b`・`/notes/99999999999999999999` を開くと、それぞれどうなるでしょうか。

**問3:** 例4-2の `module_param!` の `error = not_found` を `error = bad_request` に変えると、`/notes/0` と `/notes/abc` の返事はどう変わるでしょうか。どちらを使うのがよいか、理由も考えてください。

### 解答と解説

**問1の解答:**

```text
@@OUT:u04_q1@@
```

- `+` は空白に、`%26` は `&` に戻り、検索の言葉は `a b&c` になりました。クエリの中では、空白を `+` で、`&`（項目の区切りに使う文字）を `%26` で書く決まりだからです。`&` をそのまま書くと、`c` が別の項目の名前として読まれてしまいます。
- HTML の中では `a b&amp;c` になっています。`&` は HTML で特別な意味を持つので、`view!` が `&amp;` に書き換えました（エスケープ。UNIT 05）。ブラウザには `a b&c` と表示されます。

**問2の解答:**

```text
@@OUT:u04_q2@@
```

- `/users/a%2Fb` は、1つの区切りの中の `%2F` が `/` に戻り、名前は `a/b` になりました。区切りの `/` とは別物として扱われます。
- `/users/a/b` は、区切りが1つ多いので、`/users/{name}` に当たらず、404 です。
- `/notes/99999999999999999999` は、`u64` に入りきらない大きな数なので、変えられずに 404 でした。「数字だけ」でも読めるとは限りません。型に任せておけば、こうした端の値も断ってくれます（第11巻 UNIT 01 の境界値）。

**問3の解答:**

`src/bin/u04_q3.rs` として、`error = bad_request` に変えたものを保存しました。

```text
@@OUT:u04_q3@@
```

- どちらも 400 になり、本文は `bad request: invalid value for path parameter "id"`（パスの `id` の値が不正です）でした。
- RustNote は、**404 を選んでいます**。`/notes/0` や `/notes/abc` は「そんなノートは無い」と同じ意味で、無い ID・他人の ID と同じ返事にそろえておけば、外から「この ID の形は正しい・正しくない」の違いを探られにくくなります。クエリの `?page=abc` は、ページ自体はあるので、400（送られてきた内容がおかしい）が合います。

### まとめ

- パスの部品は `module_param!(id: 型, error = …)` で区切りを `{id}` にして、`path_param::<Id>(cx)?` で読む。型に `FromStr` を書けば、その決まり（1 以上など）が URL の読み取りにも効く。
- クエリは `#[query_params]` の構造体で読む。項目は `Option<T>` にし、無い・空は `None`。読めなかったときは、宣言の `error = …` の返事（404・400 など）になり、ページの関数は動かない。
- `cx: &Cx` があれば、どの関数からでも、メソッド・URL・ヘッダー・パス・クエリを読める。`view!` に入れられるのは文字や数などで、自分の型は `to_string()` などで変えてから入れる。
