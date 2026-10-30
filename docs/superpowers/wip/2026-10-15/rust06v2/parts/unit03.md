## UNIT 03: ルーター・page・route・layout

### この単元のゴール

- `module_router!()` で、Rust のモジュールの並びを、そのまま URL の並びにできる（`about` → `/about`、`blog_posts` → `/blog-posts`）
- 画面を返す `#[page]` と、文字や JSON を返す `#[route(GET)]` を使い分けられる
- `#[layout]` で全ページに共通の外枠（HTML の骨組み・メニュー）を作り、入れ子のレイアウトと、処理を包む `#[layer]` の順番を説明できる
- `href!` でリンク先を作る理由と、同じ URL のページが2つあるときの起動時の panic・`(slot)` の書き忘れ・`href!` の間違いを読める

### たとえ話でつかむ

大きなデパートを考えます。

- デパートは、**階**と**売り場**に分かれています。「5階の、おもちゃ売り場」のように、場所の並びで売り場が決まります。案内板は、その並びをそのまま書いています。
- どの売り場にも、**同じ建物の壁と天井**があります。売り場の人は、壁や天井を自分で作りません。建物（外枠）の中に、売り場の中身だけを並べます。
- 5階の売り場には、さらに**5階だけの飾り付け**があることもあります。建物の外枠の中に、5階の飾り付けがあり、その中に売り場があります。
- 入口には**警備員**がいて、入ってきた人と出ていった人を、全員記録します。売り場の中で何が起きても、出入りは必ず警備員の前を通ります。
- 店内の案内は、「5階のおもちゃ売り場」と**売り場の名前**で書きます。売り場が引っ越しても、名前で書いた案内なら、案内板を見れば新しい場所が分かります。

| デパート | Topcoat のアプリ |
|---|---|
| 階と売り場の並び | Rust の**モジュールの並び**（`mod about` → `/about`） |
| 案内板 | `module_router!()`（モジュールの並びから、URL の名簿を作る） |
| 売り場 | `#[page]`（画面を返す）と `#[route(GET)]`（文字や JSON を返す） |
| 建物の壁と天井 | `#[layout]`（どのページにも共通の、HTML の外枠） |
| 5階だけの飾り付け | 入れ子のレイアウト（`/settings` より下だけの外枠） |
| 入口の警備員 | `#[layer]`（要求の処理を、前と後ろで包む） |
| 売り場の名前で書いた案内 | `href!(about::about_page)`（関数の名前からリンク先を作る） |

> **用語:** モジュールルーティング（module routing：ファイルやモジュールの並びから、URL を決める書き方。Topcoat が勧める書き方で、RustNote もこれを使う）

<!-- -->

> **用語:** レイアウト（layout：いくつものページを包む、共通の外枠。HTML の `<html>`・`<head>`・メニューなどを1か所に書く）

<!-- -->

> **用語:** レイヤー（layer：要求の処理の前と後ろに、決まった仕事（時間を測る・記録する など）をはさむもの）

### 図で理解する

例3-1のアプリの、モジュールと URL の対応です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="160" y="22" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Rust のモジュール（u03_site.rs）</text>
<text x="560" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">URL</text>
<rect x="20" y="36" width="280" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="54" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">（ファイルの一番上）</text>
<text x="160" y="70" font-size="11" fill="#475569" text-anchor="middle">module_router!() を呼ぶ・#[layout]・#[page] home</text>
<rect x="60" y="88" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="110" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod about（#[page]）</text>
<rect x="60" y="134" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="156" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod blog_posts（#[page]）</text>
<rect x="60" y="180" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="202" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod api</text>
<rect x="100" y="222" width="200" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="244" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod health（#[route(GET)]）</text>
<rect x="440" y="36" width="250" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="61" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /</text>
<rect x="440" y="88" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="110" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /about</text>
<rect x="440" y="134" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="156" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /blog-posts</text>
<rect x="440" y="222" width="250" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="565" y="244" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /api/health（文字）</text>
<line x1="300" y1="56" x2="436" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="105" x2="436" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="151" x2="436" y2="151" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="239" x2="436" y2="239" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<text x="368" y="138" font-size="11" fill="#c2410c" text-anchor="middle">_ → -</text>
<text x="368" y="198" font-size="10" fill="#475569" text-anchor="middle">api は区切りだけ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: モジュールと URL の対応。module_router!() を呼んだ場所が / になり、モジュールが1つ深くなるたびに、URL の区切りが1つ増える。モジュール名の _ は - になる</div>
</div>

- `module_router!()` を呼んだモジュール（ここではファイルの一番上）が `/` になります。`mod about` の中の `#[page]` は `/about` に、`mod api` の中の `mod health` の `#[route(GET)]` は `/api/health` になります。
- **関数の名前は、URL に関係ありません**。`about_page` という名前でも、URL は `/about` です。
- モジュールの名前の `_` は、URL では `-` になります（`blog_posts` → `/blog-posts`）。URL では、単語を `-` でつなぐのがふつうだからです。

1つの要求が、レイヤー・レイアウト・ページを通る順番です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="210" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="40" y="44" font-size="13" font-weight="700" fill="#c2410c">#[layer] timing（前に時刻を記録 → 中を動かす → 後ろで時間を表示）</text>
<rect x="50" y="60" width="620" height="155" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="70" y="84" font-size="13" font-weight="700" fill="#1e40af">#[layout] root_layout（/ より下のすべて）: &lt;html&gt;・見出し・(slot)</text>
<rect x="80" y="100" width="560" height="100" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="124" font-size="13" font-weight="700" fill="#047857">#[layout] settings_layout（/settings より下だけ）: メニュー・(slot)</text>
<rect x="110" y="140" width="500" height="44" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="360" y="167" font-size="13" fill="#0f172a" text-anchor="middle">#[page] profile（GET /settings/profile）: &lt;h1&gt;プロフィール&lt;/h1&gt;</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 入れ子の順番。レイヤーがいちばん外で要求の処理を包み、その中で、広い範囲のレイアウトから狭い範囲のレイアウトへ、最後にページが入る</div>
</div>

### まず動かす最小コード

#### 例3-1: モジュールで URL を作り、レイアウトで包む

> **▼ このコードがやること（先に日本語で）:** ファイルの一番上に、全ページを包むレイアウト `root_layout` と、トップページ `home` を書きます。`mod about`・`mod blog_posts` の中にページを1つずつ、`mod api` の中の `mod health` に、文字だけを返す `#[route(GET)]` を書きます。レイアウトのメニューのリンク先は `href!` で作り、今開いているページのリンクには `aria-current="page"` と `active` のクラスを付けます。`main` では `module_router!()` でルーターを作ります。

```rust
//! 第6巻 UNIT 03: モジュールの並びを、そのまま URL の並びにする（module_router!）。
//! 実行: cargo run --bin u03_site

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, href, layout, module_router, page},
    view::{View, class, view},
};

/// すべてのページを包むレイアウト（HTML の外枠と、上のメニュー）
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    // 「この場所について」のリンク先と、いまそのページを開いているか
    let about = href!(about::about_page);
    let on_about = about.is_current(cx);
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 03 の練習"</title>
            </head>
            <body>
                <nav>
                    <a href=(href!(home))>"トップ"</a>
                    <a
                        href=(about)
                        aria-current=(on_about.then_some("page"))
                        class=(class!("nav", "active" if on_about))
                    >
                        "この場所について"
                    </a>
                    <a href=(href!(blog_posts::list))>"記事"</a>
                </nav>
                (slot)
            </body>
        </html>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    /// GET /about
    #[page]
    pub async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }
}

mod blog_posts {
    use super::*;

    /// GET /blog-posts（モジュール名の _ は - になる）
    #[page]
    pub async fn list() -> Result<impl View> {
        Ok(view! { <h1>"記事の一覧"</h1> })
    }
}

mod api {
    mod health {
        use topcoat::{Result, router::route};

        /// GET /api/health（HTML ではなく、文字だけを返す）
        #[route(GET)]
        async fn health() -> Result<&'static str> {
            Ok("ok")
        }
    }
}

#[tokio::main]
async fn main() {
    // このファイル（クレートの一番上）が / になる
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

UNIT 02 の `web_lab` の `src/bin/u03_site.rs` として保存し、起動します。

```powershell
cargo run --bin u03_site
```

2つ目の窓から、4つの URL を開きます。

```powershell
curl.exe -i http://127.0.0.1:3000/
curl.exe -i http://127.0.0.1:3000/about
curl.exe -i http://127.0.0.1:3000/blog-posts
curl.exe -i http://127.0.0.1:3000/api/health
```

**▼ 実行結果**

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 261
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a></nav><h1>トップ</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/about
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 303
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" aria-current="page" class="nav active">この場所について</a><a href="/blog-posts">記事</a></nav><h1>この場所について</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/blog-posts
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 267
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a></nav><h1>記事の一覧</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/api/health
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 2
date: Fri, 30 Oct 2026 11:37:49 GMT

ok
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: module_router!() は、モジュールの並びから名簿を作る

- `module_router!()` を呼んだ場所（このファイルの一番上）が `/` になり、その中の `#[page] home` が `GET /` に答えました。`mod about` の中のページは `/about`、`mod blog_posts` の中のページは `/blog-posts` です。
- `#[page]` に URL の文字列を書いていないことに注目してください。UNIT 02 では `#[page("/")]` と書きましたが、モジュールルーティングでは、**モジュールの場所が URL を決める**ので、書きません。
- `module_router!()` は、ファイルを探して回るわけではありません。Rust が `mod about` などの宣言でコンパイルしたモジュールだけが、名簿に入ります。別のファイル（`src/about.rs`）に分けたときは、`mod about;` の宣言を忘れないようにします（宣言を忘れると、そのページは 404 になります）。

##### 解説2: #[page] は画面、#[route(GET)] は文字や JSON

- `/api/health` は、`content-type: text/plain; charset=utf-8` で、本文は `ok` の2文字だけでした。`#[route(GET)]` の関数は、`&'static str`（文字）や `Json`（第6巻 UNIT 06）などを返し、それがそのまま応答になります。**レイアウトにも包まれません**。
- `#[page]` は、`View`（画面）を返し、レイアウトに包まれます。人が見る画面は `#[page]`、プログラムやヘルスチェックが読む応答は `#[route]`、と使い分けます。
- `#[route]` は、メソッドを必ず書きます（`#[route(GET)]`・`#[route(POST)]`、いくつかなら `#[route([GET, POST])]`）。`#[page]` は、書かなければ GET です。

##### 解説3: レイアウトは (slot) の所にページを入れる

- 4つのページのどれにも、`<nav>`（メニュー）が付いています。レイアウト `root_layout` が、`(slot)` の所に、それぞれのページの中身（`<h1>…</h1>`）を入れたからです。ページの関数には、`<html>` も `<nav>` も書いていません。
- `/about` を開いたときだけ、「この場所について」のリンクに `aria-current="page"` と `class="nav active"` が付きました。`href!(about::about_page).is_current(cx)` が「今、このリンク先を開いている」を `true` で返したからです。`aria-current="page"` は、画面の読み上げソフトに「これが今のページです」と伝える印で、`active` のクラスは、見た目（CSS）で強調するために使います。
- `class!("nav", "active" if on_about)` は、`on_about` が `true` のときだけ `active` を足して、`class` の値を作ります。`aria-current=(on_about.then_some("page"))` は、`None` のときは属性そのものを書きません（`/` の表示では、`aria-current` がありません）。

##### 解説4: リンク先は href! で作る

- メニューのリンク先は、`"/about"` と文字列で書かずに、`href!(about::about_page)` と、**ページの関数の名前**で書きました。モジュールの名前を変えて URL が変わっても、`href!` は新しい URL を作るので、リンクを直し忘れません。名前の付け間違いは、コンパイルで分かります（エラー2）。

### もう一歩: 実務に近い形

#### 例3-2: レイアウトを入れ子にし、レイヤーで時間を測る

`/settings` より下のページにだけ、設定のメニューを付けます。全部の要求には、処理にかかった時間を表示するレイヤーを付けます。

> **▼ このコードがやること（先に日本語で）:** ファイルの一番上に、全部の要求を包むレイヤー `timing`（処理の前に時刻を覚え、処理のあとで、メソッド・パス・状態コード・かかった時間を表示する）と、外側のレイアウトを書きます。`mod settings` の中に、内側のレイアウト `settings_layout` と、`/settings` のページを、さらにその中の `mod profile` に `/settings/profile` のページを書きます。

```rust
//! 第6巻 UNIT 03 の例3-2: レイアウトを入れ子にし、レイヤーで処理の時間を測る。
//! 実行: cargo run --bin u03_nested

use topcoat::{
    Result,
    context::Cx,
    router::{
        Body, Next, Slot, layer, layout, module_router, page,
        request::{method, uri},
        response::Response,
    },
    view::{View, view},
};

/// すべての要求を包むレイヤー: 処理にかかった時間を表示する
#[layer]
async fn timing(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    let status = match &result {
        Ok(response) => response.status().to_string(),
        Err(error) => format!("エラー（{error}）"),
    };
    println!(
        "{} {} -> {status}（{:?}）",
        method(cx),
        uri(cx).path(),
        started.elapsed()
    );
    result
}

/// 外側のレイアウト（/ より下の、すべてのページ）
#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <body>
                <header>"サイトの見出し"</header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod settings {
    use super::*;

    /// 内側のレイアウト（/settings より下のページだけ）
    #[layout]
    async fn settings_layout(slot: Slot<'_>) -> Result<impl View> {
        Ok(view! {
            <div class="settings">
                <nav>"設定のメニュー"</nav>
                (slot)
            </div>
        })
    }

    /// GET /settings
    #[page]
    async fn settings_top() -> Result<impl View> {
        Ok(view! { <h1>"設定"</h1> })
    }

    mod profile {
        use super::*;

        /// GET /settings/profile
        #[page]
        async fn profile() -> Result<impl View> {
            Ok(view! { <h1>"プロフィール"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u03_nested
```

**▼ 実行結果（curl.exe -i の表示。date と vary の行は省いています）**

```text
### /
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 82

<html><body><header>サイトの見出し</header><h1>トップ</h1></body></html>

### /settings
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 139

<html><body><header>サイトの見出し</header><div class="settings"><nav>設定のメニュー</nav><h1>設定</h1></div></body></html>

### /settings/profile
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 151

<html><body><header>サイトの見出し</header><div class="settings"><nav>設定のメニュー</nav><h1>プロフィール</h1></div></body></html>

### /settings/nothing
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

**▼ サーバーの窓の表示**

```text
GET / -> 200 OK（321.8µs）
GET /settings -> 200 OK（108.5µs）
GET /settings/profile -> 200 OK（110.3µs）
```

- `/settings/profile` の HTML は、外側のレイアウト（`<header>サイトの見出し</header>`）の中に、内側のレイアウト（`<div class="settings"><nav>設定のメニュー</nav>`）があり、その中にページ（`<h1>プロフィール</h1>`）が入っています。**範囲の広いレイアウトが外側、狭いレイアウトが内側**です。`/`（トップ）には、内側のレイアウトは付きません。
- レイヤーは、3つの要求のそれぞれで、時間を表示しました（数百マイクロ秒）。`next.run(cx, body).await` の前と後ろに書いたことが、ページの処理を**はさんで**動きます。
- `/settings/nothing`（無い URL）の 404 は、**サーバーの窓に表示されていません**。モジュールに書いたレイヤーは、そのモジュールより下で**見つかったルート**を包むので、どこにも当たらない要求は通らないのです。RustNote は、`not_found!()` を書いて、見つからない URL も自分の 404 の画面とアクセスログに通しています（UNIT 10）。

> **ヒント:** レイヤーは、ログ・時間の計測・決まったヘッダーを足す、のような「どの要求にも同じことをする」仕事に使います。「ログインしているか」の確かめは、レイヤーではなく、ページの最初で関数（RustNote の `require_user`）を呼ぶのが Topcoat の考え方です（第8巻 UNIT 01・UNIT 07）。

### よくあるエラーと読み方

#### エラー1: 同じ URL のページが2つあって、起動した途端に止まった

`mod about` の中に、「お問い合わせ」のページを書き足してしまった場合です（`errs/e03_conflict.rs`）。

```rust
//! 同じモジュールに、GET のページを2つ書いてしまった
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    #[page]
    async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }

    // 書き足したつもりが、同じモジュール（/about）に入れてしまった
    #[page]
    async fn contact_page() -> Result<impl View> {
        Ok(view! { <h1>"お問い合わせ"</h1> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

コンパイルは通ります。起動すると、すぐに止まります。

```text

thread 'main' (212584) panicked at src\bin\e03_conflict.rs:30:35:
duplicate route registered for `GET /about`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「`GET /about` に、ルートが二重に登録されました」という panic です。同じモジュールの `#[page]` は、どれも同じ URL（`/about`）になるので、`about_page` と `contact_page` が、同じ `GET /about` を取り合いました。30 行目の `module_router!().build()` が、名簿を作るときに見つけて止めました。
- **直し方:** 「お問い合わせ」用に `mod contact` を作り、そこにページを移します（問1）。同じモジュールに置いてよいのは、**メソッドが違う**ページだけです（RustNote の `routes/notes.rs` の `GET /notes` と `POST /notes` のように）。
- **学べること:** この間違いはコンパイルでは見つからず、**起動したとき**に分かります。名簿を作る `build()` は、起動の最初に1回だけ動くので、テスト（第11巻 UNIT 03（準備中））でルーターを組み立てるだけでも見つけられます。

#### エラー2: href! に、#[page] を付け忘れた関数を渡した（E0573）

`about_page` の `#[page]` を付け忘れたまま、`href!(about_page)` でリンクを作った場合です（`errs/e03_href.rs`）。

```rust
//! href! に、ページではない、ふつうの関数の名前を渡してしまった
use topcoat::{
    Result,
    router::{href, module_router, page},
    view::{View, view},
};

/// ページではない、ただの関数（#[page] を付け忘れた）
async fn about_page() -> Result<String> {
    Ok("この場所について".to_string())
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>"トップ"</h1>
        <a href=(href!(about_page))>"この場所について"</a>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo build --bin e03_href
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0573]: cannot find type `about_page` in this scope
  --> src\bin\e03_href.rs:17:24
   |
17 |         <a href=(href!(about_page))>"この場所について"</a>
   |                        ^^^^^^^^^^ not found in this scope
   |
   = note: a function named `about_page` exists in another namespace

For more information about this error, try `rustc --explain E0573`.
error: could not compile `web_lab` (bin "e03_href") due to 1 previous error
```

- **何が起きたか:** 「`about_page` という**型**が見つかりません」というエラーです。`note:` は「`about_page` という名前の**関数**なら、別の名前の空間にあります」と言っています。`#[page]` は、関数と同じ名前の**型**も作り、`href!` はその型から URL を作ります。`#[page]` が無いと型が作られないので、「型が無い」と言われるのです。
- **直し方:** `about_page` に `#[page]` を付けます。`href!` に渡せるのは、`#[page]`・`#[route]` などが付いた関数だけです。
- **学べること:** 文字列で `"/about"` と書いていたら、このページが無いことにコンパイルは気づけず、リンクを押したときに初めて 404 になります。`href!` なら、**リンク先が本当にあるか**をコンパイルが確かめます。

#### エラー3（エラーが出ない間違い）: レイアウトに (slot) を書き忘れた

レイアウトの `view!` に、`(slot)` を書き忘れた場合です（`errs/e03_noslot.rs`。抜き出しています）。

```rust
#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <body>
                <header>"サイトの見出し"</header>
            </body>
        </html>
    })
}
```

ビルドすると、エラーではなく、**警告**が1つ出ます。

```text
   Compiling web_lab v0.1.0 (...\web_lab)
warning: unused variable: `slot`
 --> src\bin\e03_noslot.rs:9:22
  |
9 | async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
  |                      ^^^^ help: if this is intentional, prefix it with an underscore: `_slot`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `web_lab` (bin "e03_noslot") generated 1 warning (run `cargo fix --bin "e03_noslot" -p web_lab` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.45s
```

起動して開くと、200 なのに、ページの中身（`<h1>トップ</h1>`）がありません。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 64

<html><body><header>サイトの見出し</header></body></html>
```

- **何が起きたか:** レイアウトは、受け取ったページの中身（`slot`）を、どこにも入れませんでした。どのページを開いても、見出しだけの同じ画面になります。状態コードは 200 なので、ログを見ても気づけません。
- **気づき方:** `warning: unused variable: slot`（`slot` という変数を使っていません）が、ちゃんと教えてくれていました。`help:` の「わざとなら `_slot` にしてください」に従うのではなく、**なぜ使っていないのか**を考えます。警告も、エラーと同じように読むくせを付けましょう（`cargo clippy -- -D warnings` で、警告をエラーとして扱うこともできます。第1巻 UNIT 06（準備中））。

### RustNote ではここで使う

RustNote のルーターは、`src/routes.rs` の `build_router` で作っています（抜き出しています）。

```rust
pub fn build_router(state: AppState, assets: impl Into<AssetConfig>) -> Router {
    module_router!()
        // クッキーを読み書きできるようにする（セッションとフラッシュメッセージで使う）
        .cookies()
        // セッション（ログイン）。トークンは __Host-session クッキーで運ばれる
        .sessions(SessionConfig::default())
        // CSS を /_topcoat/assets/... で配信する
        .assets(assets)
        // どのページからも app_state(cx) で借りられるようにする
        .app_context(state)
        .build()
}
```

- 例3-1と同じく、`module_router!()` で名簿を作っています。`.cookies()`・`.sessions(…)`・`.assets(…)`・`.app_context(…)` は、ページから使う道具を、ルーターに登録しています（UNIT 07・08・10）。
- `routes.rs` の先頭のコメントには、ファイルと URL の対応の表があります。`routes/notes/id/edit.rs` が `/notes/{id}/edit` になる、のように、**ファイルの並びが、そのまま URL の並び**です。`{id}` の部分は、UNIT 04で扱います。

同じファイルに、全ページを包むレイアウトと、アクセスログのレイヤーもあります（抜き出しています）。

```rust
/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
```

- 例3-2の `timing` と同じ形です。`/health`（`routes/health.rs` の `#[route(GET)]`）だけはログに出さないように、パスで分けています。公開したあとは、Fly.io が数十秒ごとに `/health` を確かめに来るので、全部書くとログが埋まってしまうからです（第12巻 UNIT 06（準備中））。
- `root_layout` は、`<head>` に CSS へのリンクを入れ、`<body>` の中に、ヘッダー（`site_header`）と、`error_boundary`（ページで起きたエラーを、エラーの画面に変える部品。UNIT 10）で包んだ `(slot)` を置いています。

### 練習問題

**問1:** 例3-1のアプリに、「お問い合わせ」のページ（`/contact`）を足し、メニューにもリンクを足してください（エラー1の正しい直し方です）。

**問2:** 例3-1のアプリで、`/about/`（最後に `/` が付いている）と、`/blog_posts`（`-` ではなく `_`）を `curl.exe -i` で開くと、それぞれどうなるでしょうか。

**問3:** 例3-1の `/api/health` に、`curl.exe -I`（HEAD）と `curl.exe -i -X POST`（POST）で要求を送ると、どうなるでしょうか。`#[route(GET)]` には、GET しか書いていません。

### 解答と解説

**問1の解答:**

`mod contact` を作り、その中に `#[page]` の関数を書き、メニューに `href!(contact::contact_page)` のリンクを足します（`src/bin/u03_q1.rs`。足した所だけを抜き出しています）。

```rust
                    <a href=(href!(blog_posts::list))>"記事"</a>
                    <a href=(href!(contact::contact_page))>"お問い合わせ"</a>
```

```rust
mod contact {
    use super::*;

    /// GET /contact（足したページ）
    #[page]
    pub async fn contact_page() -> Result<impl View> {
        Ok(view! { <h1>"お問い合わせ"</h1> })
    }
}
```

```powershell
cargo run --bin u03_q1
```

```text
### curl.exe -i http://127.0.0.1:3000/contact
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 311
date: Fri, 30 Oct 2026 11:37:53 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a><a href="/contact">お問い合わせ</a></nav><h1>お問い合わせ</h1></body></html>
```

- `main` も `module_router!()` も変えずに、`/contact` が 200 になり、メニューにリンクが増えました。モジュールを1つ足すと、URL が1つ増えます。

**問2の解答:**

```text
### curl.exe -i http://127.0.0.1:3000/about/
HTTP/1.1 308 Permanent Redirect
location: /about
content-length: 0
date: Fri, 30 Oct 2026 11:37:49 GMT



### curl.exe -i http://127.0.0.1:3000/blog_posts
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9
date: Fri, 30 Oct 2026 11:37:49 GMT

not found
```

- `/about/` は `308 Permanent Redirect` で、`location: /about` でした。Topcoat は、最後の `/` があるかないかだけが違う URL を、**書いたとおりの形**（ここでは `/` なし）へ移します。`308` は「ずっと、こちらに移りました。同じメソッドで送り直してください」という意味です（UNIT 06 の 303 と違い、POST は POST のまま送り直されます）。ブラウザは、自動で `/about` を開き直します。
- `/blog_posts` は 404 です。URL は `/blog-posts` だけで、`_` の形は用意されません。リンクを `href!` で作っていれば、この取り違えは起きません。

**問3の解答:**

```text
### curl.exe -I http://127.0.0.1:3000/api/health
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 2
date: Fri, 30 Oct 2026 11:37:49 GMT

### curl.exe -i -X POST http://127.0.0.1:3000/api/health
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Fri, 30 Oct 2026 11:37:49 GMT
```

- HEAD は 200 で、GET と同じヘッダー（`content-length: 2`）で本文がありません。`GET` を書いたルートには、Topcoat が HEAD にも自動で答えます。
- POST は `405 Method Not Allowed` と `allow: GET, HEAD` でした。UNIT 02 の `#[page]` と同じく、書いていないメソッドは、Topcoat が 405 で断ってくれます。

### まとめ

- `module_router!()` を呼んだモジュールが `/` になり、`mod` が1つ深くなるたびに URL の区切りが1つ増える。関数の名前は URL に関係なく、モジュール名の `_` は `-` になる。
- 画面は `#[page]`（レイアウトに包まれる）、文字や JSON は `#[route(GET)]`（そのまま応答になる）。リンクは `href!` で関数の名前から作り、無いページへのリンクをコンパイルで見つける。
- レイアウトは広い範囲が外側、狭い範囲が内側に入れ子になり、`(slot)` の所にページが入る。レイヤーは処理を前と後ろではさむ。同じ URL・同じメソッドのページが2つあると、起動したときに panic で止まる。
