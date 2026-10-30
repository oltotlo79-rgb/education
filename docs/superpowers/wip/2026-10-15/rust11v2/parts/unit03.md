## UNIT 03: HTTP のテスト

### この単元のゴール

- レスポンスの4つの部分（状態コード・ヘッダー・クッキー・本文）を、RustNote の本物のレスポンスで読み、それぞれ何を確かめればよいかを言える
- 確かめの道具（`assert_redirect`・`cookie_attributes`）を作り、`#[track_caller]` を付けて、失敗したときに**テストのどの行か**が出るようにできる
- 本文を「その文字があるか」（`contains`）ではなく、CSS セレクター（`scraper`）で「どの要素の中か」まで決めて確かめる理由を、通ってしまう間違いで説明できる
- 同じ名前のヘッダー（`Set-Cookie`）が2つあるときの読み方と、順番に頼ったテストが、ときどきだけ失敗する理由を言える

### たとえ話でつかむ

通販の荷物を受け取って、中身を確かめる（検品する）場面を考えます。

- **伝票の印:** 「お届け済み」「転送しました」のような印が、まず目に入ります。これが**状態コード**です。「転送しました」なら、**転送先の住所**（`Location`）も確かめます。
- **同封の会員証:** 箱に、会員証が入っていることがあります。会員証には「有効期限」「本人だけが使えます」のような**注意書き**（属性）が付いています。会員証が2枚入っているとき、どちらが上に入っているかは、**決まっていません**。だから、上から1枚目ではなく、**名前で**探します。
- **箱の中身:** 「箱のどこかに『リンゴ』という字がある」では、検品になりません。中に入っていたチラシに「リンゴ」と書いてあるだけかもしれないからです。「**果物の仕切りの中に**、リンゴが1個」と、**場所まで決めて**確かめます。

| 荷物の検品 | HTTP のテスト |
|---|---|
| 伝票の印 | 状態コード（`200 OK`・`303 See Other`・`404 Not Found` など） |
| 転送先の住所 | `Location` ヘッダー |
| 会員証と注意書き | `Set-Cookie` ヘッダーと、その属性（`HttpOnly`・`Max-Age` など） |
| 会員証が2枚 | 同じ名前のヘッダーが2つ（`get_all` で全部読み、名前で探す） |
| 箱の中身 | 本文（HTML） |
| 仕切りの中で確かめる | CSS セレクターで、要素を決めて確かめる（例3-3） |

> **用語:** ヘッダー（レスポンスの本文の前に付く「名前: 値」の行。移動先の `Location`、クッキーを渡す `Set-Cookie`、本文の形を表す `Content-Type` など。同じ名前の行が、いくつあってもよい）

<!-- -->

> **用語:** CSS セレクター（HTML の中の要素を、`ul.note-list li a`（`note-list` という class の `ul` の中の、`li` の中の `a`）のように、場所で指定する書き方。CSS で見た目を当てるときと同じ書き方）

<!-- -->

> **用語:** `#[track_caller]`（関数に付ける印。その関数の中で失敗（panic）したとき、失敗の場所として、その関数を**呼んだ行**が表示されるようになる）

### 図で理解する

RustNote の `POST /signup` のレスポンスの4つの部分と、それぞれの確かめ方です（例3-1の出力から）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="16" width="420" height="268" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<rect x="36" y="30" width="388" height="34" rx="4" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="48" y="52" font-size="12" fill="#0f172a" font-family="Consolas, monospace">303 See Other</text>
<rect x="36" y="74" width="388" height="34" rx="4" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="48" y="96" font-size="12" fill="#0f172a" font-family="Consolas, monospace">location: /notes</text>
<rect x="36" y="118" width="388" height="78" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="48" y="140" font-size="11" fill="#0f172a" font-family="Consolas, monospace">set-cookie: flash=signed_up; HttpOnly; ...</text>
<text x="48" y="162" font-size="11" fill="#0f172a" font-family="Consolas, monospace">set-cookie: __Host-session=...; HttpOnly;</text>
<text x="60" y="182" font-size="11" fill="#0f172a" font-family="Consolas, monospace">SameSite=Lax; Secure; Path=/; Max-Age=2592000</text>
<rect x="36" y="206" width="388" height="64" rx="4" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="48" y="228" font-size="11" fill="#0f172a">本文（303 のときは空。GET /notes なら HTML）</text>
<text x="48" y="250" font-size="11" fill="#0f172a" font-family="Consolas, monospace">&lt;ul class="note-list"&gt;&lt;li&gt;&lt;a&gt;...&lt;/a&gt;</text>
<text x="460" y="44" font-size="12" font-weight="700" fill="#1d4ed8">状態コード</text>
<text x="460" y="60" font-size="11" fill="#0f172a">assert_eq!(status, SEE_OTHER)</text>
<text x="460" y="88" font-size="12" font-weight="700" fill="#047857">Location（移動先）</text>
<text x="460" y="104" font-size="11" fill="#0f172a">assert_redirect（例3-2）</text>
<text x="460" y="140" font-size="12" font-weight="700" fill="#c2410c">Set-Cookie（2つ）</text>
<text x="460" y="158" font-size="11" fill="#0f172a">get_all で全部読み、名前で探す</text>
<text x="460" y="176" font-size="11" fill="#b91c1c">並ぶ順番は、決まっていない</text>
<text x="460" y="226" font-size="12" font-weight="700" fill="#6d28d9">本文</text>
<text x="460" y="244" font-size="11" fill="#0f172a">CSS セレクターで要素を決める</text>
<text x="460" y="262" font-size="11" fill="#0f172a">（例3-3・scraper）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: レスポンスの4つの部分。状態コードと Location は値で、Set-Cookie は名前で探して属性まで、本文は要素を決めて確かめる</div>
</div>

検索の画面で、「買う」という字が、どこにあるかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="360" height="220" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="36" y="40" font-size="12" font-weight="700" fill="#0f172a">GET /notes?q=買う の本文</text>
<rect x="36" y="54" width="328" height="44" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="48" y="74" font-size="11" fill="#0f172a">検索の欄（入力を、そのまま戻して表示）</text>
<text x="48" y="90" font-size="11" fill="#0f172a" font-family="Consolas, monospace">&lt;input name="q" value="買う"&gt;</text>
<rect x="36" y="110" width="328" height="66" rx="4" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="48" y="130" font-size="11" fill="#0f172a">一覧（検索で見つかったノート）</text>
<text x="48" y="148" font-size="11" fill="#0f172a" font-family="Consolas, monospace">&lt;ul class="note-list"&gt;</text>
<text x="60" y="166" font-size="11" fill="#0f172a" font-family="Consolas, monospace">&lt;li&gt;&lt;a&gt;牛乳を買う&lt;/a&gt;</text>
<text x="48" y="200" font-size="11" fill="#475569">見つからなかったときは、一覧の代わりに</text>
<text x="48" y="218" font-size="11" fill="#475569">「条件に合うノートはありません。」</text>
<text x="410" y="70" font-size="12" font-weight="700" fill="#c2410c">contains("買う")</text>
<text x="410" y="88" font-size="11" fill="#0f172a">本文のどこかにあれば true</text>
<text x="410" y="106" font-size="11" fill="#b91c1c">検索の欄にも「買う」があるので、</text>
<text x="410" y="122" font-size="11" fill="#b91c1c">1件も見つからなくても true（エラー3）</text>
<text x="410" y="160" font-size="12" font-weight="700" fill="#047857">ul.note-list li a の文字</text>
<text x="410" y="178" font-size="11" fill="#0f172a">一覧のリンクだけを、順に取り出す</text>
<text x="410" y="196" font-size="11" fill="#0f172a">["牛乳を買う"] と、まるごと比べる</text>
<line x1="406" y1="80" x2="368" y2="78" stroke="#f97316" stroke-width="2" marker-end="url(#ar-11-0301)"/>
<line x1="406" y1="80" x2="368" y2="150" stroke="#f97316" stroke-width="2" marker-end="url(#ar-11-0301)"/>
<line x1="406" y1="170" x2="368" y2="162" stroke="#10b981" stroke-width="2" marker-end="url(#ar-11-0301)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 検索の画面の「買う」。contains は検索の欄と一覧の両方を見てしまうので、一覧が空でも通る。CSS セレクターなら、一覧のリンクだけを見る</div>
</div>

### まず動かす最小コード

#### 例3-1: 1つのレスポンスの、4つの部分を見る

テストを書く前に、RustNote が**本当に何を返しているか**を、全部表示して見ます。

> **▼ このコードがやること（先に日本語で）:** 登録のフォームを送り（`POST /signup`）、返ってきた状態コードと、ヘッダーを全部表示します。セッションのトークンは、先頭の6文字と長さだけにします。次に、そのクッキーでノートの一覧（`GET /notes`）を開き、状態コード・ヘッダーと、本文の長さ・`<title>`・`<h1>` を表示します。

```rust
//! 第11巻 UNIT 03: 1つのレスポンスの4つの部分（状態コード・ヘッダー・クッキー・本文）を見る。
//! 実行: cargo test --test u03_parts -- --nocapture

mod common;

use common::{body_text, encode, session_cookie, test_app};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

/// 状態コードと、ヘッダーを全部表示する（セッションのトークンは、先頭の6文字だけ）
fn show(label: &str, response: &Response) {
    println!("--- {label}");
    println!("状態コード: {}", response.status());
    for (name, value) in response.headers() {
        let value = value.to_str().unwrap_or("（文字にできない）");
        let shown = match value.strip_prefix("__Host-session=") {
            Some(rest) => {
                let (token, attributes) = rest.split_once(';').unwrap_or((rest, ""));
                format!(
                    "__Host-session={}…（{} 文字）;{attributes}",
                    &token[..6],
                    token.len()
                )
            }
            None => value.to_string(),
        };
        println!("{name}: {shown}");
    }
}

#[tokio::test]
async fn four_parts_of_a_response() {
    let app = test_app().await;

    // 1. 登録のフォームを送る（成功すると、303 でノートの一覧へ）
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;
    show("POST /signup", &response);
    let cookie = session_cookie(&response).unwrap();

    // 2. ログインしたクッキーで、ノートの一覧を開く
    let response = app.get("/notes", Some(&cookie)).await;
    show("GET /notes", &response);
    let html = body_text(response).await;
    println!("本文: {} バイト", html.len());
    for tag in ["<title>", "<h1>"] {
        let start = html.find(tag).unwrap();
        let end = start + html[start..].find("</").unwrap();
        println!("本文の {tag}: {}", &html[start + tag.len()..end]);
    }
}
```

コピーした `rustnote` フォルダの `tests/u03_parts.rs` として保存して実行します。

```powershell
cargo test --test u03_parts -- --nocapture
```

**▼ 実行結果**

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running tests\u03_parts.rs (target\debug\deps\u03_parts-76b358a1ad142f58.exe)

running 1 test
POST /signup -> 303 See Other (511.5942ms)
--- POST /signup
状態コード: 303 See Other
location: /notes
set-cookie: flash=signed_up; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
set-cookie: __Host-session=Vr9LDo…（46 文字）; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000
GET /notes -> 200 OK (9.328ms)
--- GET /notes
状態コード: 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
本文: 1318 バイト
本文の <title>: RustNote
本文の <h1>: ノート
test four_parts_of_a_response ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 状態コードと Location

- 登録に成功すると、`303 See Other` と `location: /notes` が返りました。ブラウザは、この2つを見て、`/notes` を開き直します（PRG。第10巻 UNIT 02）。本文は空です。
- `303` だけを確かめても、移動先が違えば、利用者は別の画面に行ってしまいます。**303 と Location は、いつも組で**確かめます（例3-2の `assert_redirect`）。

##### 解説2: Set-Cookie は2つ。属性まで読む

- `set-cookie:` の行が2つありました。お知らせの `flash=signed_up` と、ログインの `__Host-session` です。同じ名前のヘッダーが、2回出てきます。
- `__Host-session` の属性は、`HttpOnly`（JavaScript から読めない）・`SameSite=Lax`（ほかのサイトからの POST では送られない）・`Secure`（HTTPS だけ）・`Path=/`・`Max-Age=2592000`（2,592,000 秒＝30 日）です。`flash` は `Max-Age=300`（5 分）で、`Secure` がありません。どれも第8巻で確かめた守りなので、テストで、属性まで確かめます。
- この出力では `flash` が先ですが、**先とは限りません**（エラー2）。

##### 解説3: GET /notes のヘッダーと本文

- `content-type: text/html; charset=utf-8` は、「本文は HTML で、文字は UTF-8」という意味です。RustNote は、HTML の中にも `<meta charset="utf-8">` を書いていますが、ブラウザは、ヘッダーのほうを先に見ます。
- `vary:` は、Topcoat が付けた、「`Accept`・`accept-encoding` のヘッダーによって、返す中身が変わることがある」というキャッシュ向けの知らせです。
- 本文は 1,318 バイトで、`<title>` は `RustNote`、`<h1>` は `ノート` でした。
- 逆に、**無いヘッダー**もあります。`X-Frame-Options` のような、守りのためのヘッダーは1つもありませんでした（問2）。全部表示してみると、こうした「無いもの」にも気づけます。

### もう一歩: 実務に近い形

#### 例3-2: 確かめの道具を作り、`#[track_caller]` を付ける

例3-1で見た値を、`assert` で確かめます。「303 で、この URL へ」「このクッキーの属性は、これ」は何度も出てくるので、**道具**（小さな関数）にします。

> **▼ このコードがやること（先に日本語で）:** `assert_redirect` は、「303 で、`to` へ移動するか」を確かめます。`cookie_attributes` は、`Set-Cookie` を**全部**読んで、名前が合うものを探し、属性を並んだ順に返します。この2つの道具で、登録のレスポンス・ログインしていない人の移動先・Content-Type を確かめる、3つのテストを書きます。

```rust
//! 第11巻 UNIT 03: レスポンスを確かめる道具（ヘルパー）を作り、HTTP のテストを短く書く。
//! 道具に #[track_caller] を付けて、失敗したときに「テストのどの行か」が出るようにする。
//!
//! 実行: cargo test --test u03_helpers

mod common;

use common::{encode, location, test_app};
use topcoat::router::{StatusCode, header, response::Response};

const PASSWORD: &str = "correct horse battery";

/// 303 で `to` へ移動するレスポンスか
#[track_caller]
fn assert_redirect(response: &Response, to: &str) {
    assert_eq!(response.status(), StatusCode::SEE_OTHER, "303 のはず");
    assert_eq!(location(response), to, "移動先");
}

/// `name` のクッキーの属性（`HttpOnly`・`Path=/` など）を、並んだ順に返す。無ければ止まる
#[track_caller]
fn cookie_attributes(response: &Response, name: &str) -> Vec<String> {
    let prefix = format!("{name}=");
    let value = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with(&prefix))
        .unwrap_or_else(|| panic!("{name} のクッキーがありません"));
    value
        .split(';')
        .skip(1)
        .map(|attribute| attribute.trim().to_string())
        .collect()
}

#[tokio::test]
async fn signup_sets_two_cookies_and_redirects() {
    let app = test_app().await;
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;

    assert_redirect(&response, "/notes");
    // セッション: JavaScript から読めず・HTTPS だけ・ほかのサイトからの POST では送られず・30 日
    assert_eq!(
        cookie_attributes(&response, "__Host-session"),
        [
            "HttpOnly",
            "SameSite=Lax",
            "Secure",
            "Path=/",
            "Max-Age=2592000"
        ]
    );
    // お知らせ: 5 分だけ
    assert_eq!(
        cookie_attributes(&response, "flash"),
        ["HttpOnly", "SameSite=Lax", "Path=/", "Max-Age=300"]
    );
}

#[tokio::test]
async fn pages_for_members_send_guests_to_login() {
    let app = test_app().await;
    for path in ["/notes", "/notes/1", "/notes/1/edit"] {
        let response = app.get(path, None).await;
        assert_redirect(&response, "/login");
    }
}

#[tokio::test]
async fn html_pages_say_utf8() {
    let app = test_app().await;
    let response = app.get("/login", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );
}
```

```powershell
cargo test --test u03_helpers
```

**▼ 実行結果**

```text
running 3 tests
test html_pages_say_utf8 ... ok
test pages_for_members_send_guests_to_login ... ok
test signup_sets_two_cookies_and_redirects ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

- 3つとも通りました。`cookie_attributes(&response, "__Host-session")` と書けば、何を確かめているかが、読む人にもすぐ分かります。
- 道具の頭に付けた `#[track_caller]` は、**失敗したときのため**の印です。違いを見るために、`/notes/1` の移動先を、わざと `/notes` と書きまちがえたテストを、印の無い版と有る版で動かします（`assert_redirect` を3回呼び、2回目だけが間違い）。

```rust
//! 第11巻 UNIT 03 例3-2: 道具に #[track_caller] が無いと、失敗の場所が道具の中になる。
//! 3つの URL のうち、どれか1つの期待を、わざと書きまちがえている。
//! 実行: cargo test --test e03_no_track

mod common;

use common::{location, test_app};
use topcoat::router::{StatusCode, response::Response};

fn assert_redirect(response: &Response, to: &str) {
    assert_eq!(response.status(), StatusCode::SEE_OTHER, "303 のはず");
    assert_eq!(location(response), to, "移動先");
}

#[tokio::test]
async fn guests_are_redirected() {
    let app = test_app().await;
    assert_redirect(&app.get("/notes", None).await, "/login");
    assert_redirect(&app.get("/notes/1", None).await, "/notes");
    assert_redirect(&app.get("/notes/1/edit", None).await, "/login");
}
```

印が無い版（`tests/e03_no_track.rs`）:

```text
thread 'guests_are_redirected' (218940) panicked at tests\e03_no_track.rs:12:5:
assertion `left == right` failed: 移動先
  left: "/login"
 right: "/notes"
```

印が有る版（`tests/e03_track.rs`。違うのは `#[track_caller]` の1行だけ）:

```text
thread 'guests_are_redirected' (217604) panicked at tests\e03_track.rs:20:5:
assertion `left == right` failed: 移動先
  left: "/login"
 right: "/notes"
```

- 印が無いと、失敗の場所は `e03_no_track.rs:12:5`、つまり**道具の中**の `assert_eq!` の行です。3回呼んだうちの、どれが失敗したのかが分かりません。
- 印が有ると、`e03_track.rs:20:5`、つまり**2回目に呼んだ行**が出ます。道具を作ったら、`#[track_caller]` を付けるのを、くせにします。

#### 例3-3: 本文を、CSS セレクターで確かめる

本文の HTML を、文字の並びのまま調べるのではなく、**HTML として読んで**、要素を決めて調べます。HTML を読むクレート `scraper` を、練習用のコピーの dev-dependency（テストのときだけ使う依存）に足します。

```powershell
cargo add --dev scraper@=0.29.0
```

```text
    Updating crates.io index
      Adding scraper =0.29.0 to dev-dependencies
             Features:
             + errors
             + main
             - atomic
             - deterministic
             - serde
    Updating crates.io index
     Locking 28 packages to highest Rust 1.99 compatible versions
```

（このあと、`scraper` 自身と、`scraper` が使うクレートの、合わせて 28 個の `Adding` の行が続きます。）

> **▼ このコードがやること（先に日本語で）:** alice で「牛乳を買う」「本を返す」の2つのノートを作り、「買う」で検索した一覧の画面を、2つのやり方で確かめます。1つ目は、本文に「買う」という字があるか（`contains`）。2つ目は、`ul.note-list li a`（一覧のリンク）の文字を、上から順に取り出し、`["牛乳を買う"]` とまるごと比べます。

```rust
//! 第11巻 UNIT 03: 本文の確かめ方を比べる。
//! 「本文に、その文字があるか」（contains）と、「一覧のリンクに、その題があるか」（scraper の CSS セレクター）。
//!
//! 実行: cargo test --test u03_body -- --nocapture

mod common;

use common::{body_text, test_app};
use scraper::{Html, Selector};

const PASSWORD: &str = "correct horse battery";

/// 一覧のページの、ノートのリンクの文字を、上から順に返す
fn listed_titles(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let links = Selector::parse("ul.note-list li a").unwrap();
    document
        .select(&links)
        .map(|link| link.text().collect::<String>())
        .collect()
}

#[tokio::test]
async fn search_by_contains() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "本を返す", "").await;

    let html = body_text(app.get("/notes?q=%E8%B2%B7%E3%81%86", Some(&alice)).await).await;
    println!("contains(\"買う\") → {}", html.contains("買う"));
    assert!(
        html.contains("買う"),
        "「買う」で検索すると、本文に「買う」がある"
    );
}

#[tokio::test]
async fn search_by_selector() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "本を返す", "").await;

    let html = body_text(app.get("/notes?q=%E8%B2%B7%E3%81%86", Some(&alice)).await).await;
    let titles = listed_titles(&html);
    println!("一覧のリンク → {titles:?}");
    assert_eq!(
        titles,
        ["牛乳を買う"],
        "「買う」で検索すると、一覧は「牛乳を買う」だけ"
    );
}
```

```powershell
cargo test --test u03_body -- --nocapture --test-threads=1
```

**▼ 実行結果**

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running tests\u03_body.rs (target\debug\deps\u03_body-3a77394819c142a3.exe)

running 2 tests
test search_by_contains ... POST /signup -> 303 See Other (425.0911ms)
POST /notes -> 303 See Other (7.3585ms)
POST /notes -> 303 See Other (9.0424ms)
GET /notes -> 200 OK (5.4579ms)
contains("買う") → true
ok
test search_by_selector ... POST /signup -> 303 See Other (410.4965ms)
POST /notes -> 303 See Other (6.984ms)
POST /notes -> 303 See Other (8.6498ms)
GET /notes -> 200 OK (3.9642ms)
一覧のリンク → ["牛乳を買う"]
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s
```

- どちらも通りました。ここだけ見ると、同じに見えます。でも、確かめている中身は違います（図の2つ目）。違いは、検索がまちがっていたときに出ます（エラー3）。
- `--test-threads=1` は、2つのテストを1つずつ動かす指定です。同時に動かすと、アクセスログの行と `println!` の行が、行の途中で混ざって読みにくくなるので、ここでは1つずつにしました。
- `link.text()` は、要素の中の文字を返します。HTML の `&amp;` のような書き方は、もとの文字（`&`）に戻ります（問3）。

### よくあるエラーと読み方

#### エラー1: 本文を読んだあとで、状態コードを見ようとした（E0382）

```rust
//! 第11巻 UNIT 03 エラー1: 本文を読んだあとで、同じレスポンスの状態コードを見ようとした。
//! 実行: cargo test --test e03_moved

mod common;

use common::{body_text, test_app};
use topcoat::router::StatusCode;

#[tokio::test]
async fn login_page_has_a_form() {
    let app = test_app().await;
    let response = app.get("/login", None).await;

    let html = body_text(response).await;
    assert!(html.contains("<form"));
    assert_eq!(response.status(), StatusCode::OK);
}
```

```powershell
cargo test --test e03_moved
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0382]: borrow of moved value: `response`
   --> tests\e03_moved.rs:16:16
    |
 12 |     let response = app.get("/login", None).await;
    |         -------- move occurs because `response` has type `http::response::Response<Body>`, which does not implement the `Copy` trait
 13 |
 14 |     let html = body_text(response).await;
    |                          -------- value moved here
 15 |     assert!(html.contains("<form"));
 16 |     assert_eq!(response.status(), StatusCode::OK);
    |                ^^^^^^^^ value borrowed here after move
    |
note: consider changing this parameter type in function `body_text` to borrow instead if owning the value isn't necessary
   --> tests\common\mod.rs:117:34
    |
117 | pub async fn body_text(response: Response) -> String {
    |              ---------           ^^^^^^^^ this parameter takes ownership of the value
    |              |
    |              in this function

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustnote` (test "e03_moved") due to 1 previous error
```

- **何が起きたか:** 「`response` は、14 行目で `body_text` に**移された**（move）ので、16 行目では、もう使えません」というエラーです。`body_text(response: Response)` は、レスポンスを引数として**受け取って**（所有権ごと）本文を読みます（第3巻）。
- **直し方:** 本文を読む**前に**、状態コードやヘッダーを、変数に取っておきます。

```rust
    let status = response.status();
    let html = body_text(response).await;
    assert_eq!(status, StatusCode::OK);
```

問1の答えの `tests/u03_q1.rs` も、この順番で書いています。

#### エラー2（ときどき失敗する）: 1つ目の Set-Cookie を、セッションのクッキーだと思い込んだ

```rust
//! 第11巻 UNIT 03 エラー2: 1つ目の Set-Cookie が、セッションのクッキーだと思い込んだテスト。
//! 実行: cargo test --test e03_first_cookie

mod common;

use common::{encode, test_app};
use topcoat::router::header;

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn first_set_cookie_is_the_session() {
    let app = test_app().await;
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;

    // get は、同じ名前のヘッダーが2つあっても、1つ目しか返さない
    let first = response.headers()[header::SET_COOKIE].to_str().unwrap();
    assert!(
        first.starts_with("__Host-session="),
        "1つ目の Set-Cookie: {first}"
    );
}
```

このテストを、10 回続けて動かしました。

```text
1 回目: ok
2 回目: ok
3 回目: ok
4 回目: ok
5 回目: ok
6 回目: ok
7 回目: ok
8 回目: ok
9 回目: ok
10 回目: FAILED
```

失敗した回の出力です。

```text
test first_set_cookie_is_the_session ... FAILED
thread 'first_set_cookie_is_the_session' (213976) panicked at tests\e03_first_cookie.rs:24:5:
1つ目の Set-Cookie: flash=signed_up; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s
```

- **何が起きたか:** 10 回のうち、9 回は通り、1 回だけ失敗しました。失敗した回は、1つ目の `Set-Cookie` が `flash=signed_up` でした。もう一度 10 回試したときは、6 回失敗しました。RustNote は、1回も変わっていません。
- **なぜか:** `headers()[…]`（と `headers().get(…)`）は、同じ名前のヘッダーが2つあっても、**1つ目だけ**を返します。そして、Topcoat が使っている cookie 0.18.2 は、返すクッキーを `HashSet`（順番を持たない入れ物）にしまっています。`HashSet` の中の順番は、プログラムを動かすたびに変わるので、どちらが1つ目になるかは、その時しだいです。
- **直し方:** 例3-2の `cookie_attributes` のように、`get_all` で**全部**読んで、**名前で**探します。RustNote の `tests/common/mod.rs` の `session_cookie` も、この形です。
- **学べること:** 「ときどきだけ失敗するテスト」は、順番の決まっていないもの（`HashSet`・`HashMap` の並び・同時に動くテストの順番・時刻）に頼っていることがよくあります（UNIT 02 のエラー2も、同時に動くテストの順番でした）。

#### エラー3（通ってしまう間違い）: 検索を「前に一致」にしてしまった

検索の模様を作る行で、`format!("%{}%", …)`（どこかに含む）を、`format!("{}%", …)`（その字で**始まる**）と書きまちがえた場合です（練習用のコピーの `src/repository/notes.rs` で試し、必ず元に戻します）。

```rust
            Some(keyword) => format!("{}%", escape_like(keyword)),
```

例3-3のテストを動かします。

```text
test search_by_contains ... POST /signup -> 303 See Other (423.5922ms)
contains("買う") → true
test search_by_selector ... POST /signup -> 303 See Other (440.359ms)
一覧のリンク → []
thread 'search_by_selector' (198124) panicked at tests\u03_body.rs:48:5:
assertion `left == right` failed: 「買う」で検索すると、一覧は「牛乳を買う」だけ
  left: []
 right: ["牛乳を買う"]
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.99s
```

- **何が起きたか:** 「牛乳を買う」は「買う」で**始まらない**ので、一覧は空（`[]`）になりました。それなのに、`contains("買う")` の版は `true` のまま**通りました**。検索の欄に、入力した「買う」が、そのまま戻して表示されているからです。セレクターの版だけが、「一覧は `[]`」と止まりました。
- **RustNote のテストでは:** 同じ書きまちがいのまま、RustNote の `tests/http.rs` と `tests/repository.rs` も動かしました。

```text
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.01s
test search_filters_by_keyword_and_paginates ... FAILED
thread 'search_filters_by_keyword_and_paginates' (211964) panicked at tests\repository.rs:203:5:
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
```

- `tests/http.rs` の 14 個は、**全部通りました**。検索のテスト（`list_searches_and_paginates`）は「所有権」で探しますが、そのノートの本文は「所有権」そのもので、「所有権」で**始まる**ので、前に一致でも見つかるからです。`tests/repository.rs` の検索のテストが、203 行目（`_計算` で探す所）で気づきました。
- **学べること:** 本文の確かめは、「どの要素の中に、何が、いくつあるか」まで決めます。そして、テストに使うデータは、間違いが表に出る形（キーワードが**真ん中**にある題）を選びます。HTTP のテストと、リポジトリの結合テストは、おたがいに見落としを拾い合います。

### RustNote ではここで使う

- **HTTP のテストの形:** RustNote の `tests/http.rs` は、ポートを開かずに、`Router::handle` へリクエストを直接渡して確かめます（`tests/common/mod.rs` の `test_app`・`get`・`post_form`）。この単元のテストも、同じ道具を使いました。
- **Set-Cookie:** `common::session_cookie` は、`get_all` で全部読み、`__Host-session=` で始まるものを探します（エラー2の直し方の形）。一方、`notes_require_login` は、`get(header::SET_COOKIE)` で1つ目を読んでいます。ログインしていない人へのレスポンスには、クッキーが `flash` の1つしかないので、今は正しく動きますが、クッキーが増えると、エラー2と同じことが起きます（改善点）。
- **本文:** `tests/http.rs` の本文の確かめは、ほとんどが `contains` です。一覧の件数は `html.matches("<li class=").count()` で数えています。エラー3のように、`contains` が通ってしまう所が無いかを、`scraper` で見直す余地があります（改善点。RustNote の `Cargo.toml` には、`scraper` を足していません）。
- **守りのヘッダー:** RustNote は、`X-Frame-Options` などの守りのヘッダーを返していません（問2、改善点）。

### 練習問題

**問1:** ログインしていない人が `GET /notes` を開いたときの、状態コード・`Location`・`Set-Cookie`・本文を、例3-1のように表示し、`assert` で確かめてください。移動先のログイン画面に「ログインしてください」が出ることも確かめてください。

**問2:** `GET /login` と、ログインした人の `GET /notes` のレスポンスに、次の6つのヘッダーがあるかを表示してください。`x-content-type-options`・`x-frame-options`・`content-security-policy`・`referrer-policy`・`strict-transport-security`・`cache-control`。無いと、どんなことが起こりうるでしょうか。

**問3:** 題が `<b>太字</b> & "引用"` のノートを作り、一覧の画面で、`ul.note-list li a` の文字が、入力した題と同じであることと、一覧の中に `b` 要素が1つも無いことを、`scraper` で確かめてください。本文の中では、どんな形で書かれているでしょうか。

### 解答と解説

**問1の解答:**

```rust
//! 第11巻 UNIT 03 問1: ログインしていない人が GET /notes を開いたときの、4つの部分を確かめる。
//! 実行: cargo test --test u03_q1 -- --nocapture

mod common;

use common::{body_text, location, test_app};
use topcoat::router::{StatusCode, header};

#[tokio::test]
async fn guest_opens_notes() {
    let app = test_app().await;
    let response = app.get("/notes", None).await;

    let status = response.status();
    let to = location(&response).to_string();
    let cookies: Vec<String> = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|value| value.to_str().unwrap().to_string())
        .collect();
    let html = body_text(response).await;
    println!("状態コード: {status}");
    println!("Location: {to}");
    println!("Set-Cookie: {cookies:?}");
    println!("本文: {} バイト", html.len());

    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(to, "/login");
    assert_eq!(
        cookies,
        ["flash=login_required; HttpOnly; SameSite=Lax; Path=/; Max-Age=300"]
    );

    // 移動先のログイン画面で、お知らせが出る
    let html = body_text(app.get("/login", Some("flash=login_required")).await).await;
    println!(
        "移動先に「ログインしてください」: {}",
        html.contains("ログインしてください")
    );
}
```

```powershell
cargo test --test u03_q1 -- --nocapture
```

```text
状態コード: 303 See Other
Location: /login
Set-Cookie: ["flash=login_required; HttpOnly; SameSite=Lax; Path=/; Max-Age=300"]
本文: 0 バイト
移動先に「ログインしてください」: true
```

- `303`・`/login`・`flash=login_required`（5 分）の3つが、ログインの守り（`require_user`）の「出口」です。本文は 0 バイトでした。
- エラー1のとおり、`status`・`to`・`cookies` を、`body_text` の**前に**取り出しています。
- 移動先の確かめでは、`Set-Cookie` の値の `flash=login_required` を、そのまま `Cookie` として送りました。ブラウザが次のリクエストでするのと同じです。

**問2の解答:**

```rust
//! 第11巻 UNIT 03 問2: 守りに関わるヘッダーが、レスポンスにあるかを確かめる。
//! 実行: cargo test --test u03_q2 -- --nocapture

mod common;

use common::test_app;

const NAMES: [&str; 6] = [
    "x-content-type-options",
    "x-frame-options",
    "content-security-policy",
    "referrer-policy",
    "strict-transport-security",
    "cache-control",
];

#[tokio::test]
async fn security_headers() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for (path, cookie) in [("/login", None), ("/notes", Some(alice.as_str()))] {
        let response = app.get(path, cookie).await;
        println!("--- GET {path}（{}）", response.status());
        for name in NAMES {
            let value = response.headers().get(name);
            println!(
                "{name:<26} {}",
                value.map_or("無い".to_string(), |v| v.to_str().unwrap().to_string())
            );
        }
    }
}
```

```text
--- GET /login（200 OK）
x-content-type-options     無い
x-frame-options            無い
content-security-policy    無い
referrer-policy            無い
strict-transport-security  無い
cache-control              無い
--- GET /notes（200 OK）
x-content-type-options     無い
x-frame-options            無い
content-security-policy    無い
referrer-policy            無い
strict-transport-security  無い
cache-control              無い
```

- 6つとも、どちらのページにもありませんでした。それぞれ、次のことを防ぐためのヘッダーです。

| ヘッダー | 無いと、起こりうること |
|---|---|
| `x-content-type-options: nosniff` | ブラウザが、`Content-Type` と違う種類のファイルだと推し量って、扱ってしまう |
| `x-frame-options`（または CSP の `frame-ancestors`） | ほかのサイトが、RustNote の画面を `iframe` の中に見えないように重ね、利用者に、知らずにボタンを押させる（クリックジャッキング） |
| `content-security-policy` | もし HTML に、ほかの人のスクリプトが紛れ込んでも（XSS）、ブラウザが止めない |
| `referrer-policy` | ほかのサイトへのリンクを押したとき、元の URL をどこまで相手に伝えるかが、ブラウザの既定まかせになる（今の多くのブラウザは、ほかのサイトにはドメインまでしか伝えないが、古いブラウザでは `/notes?q=…` の検索の言葉まで伝わる） |
| `strict-transport-security` | 利用者が `http://` で開くたびに、HTTPS へ移る前の最初のリクエストが、暗号化されないまま送られる（途中で書き換えられるおそれ） |
| `cache-control: no-store` | ログインした人だけの画面が、ブラウザのキャッシュや履歴（戻るボタン）に残ることがある |

- RustNote は、第8巻で、クッキーの `SameSite=Lax`（ほかのサイトの `iframe` の中では、ログインのクッキーが送られない）と、Topcoat のエスケープ（問3）で、いちばん大きな危険を防いでいます。それでも、何重にも守るために、これらのヘッダーを足す余地があります（改善点）。どの値にするかは、UNIT 06（準備中）の守りの回帰テストで、もう一度考えます。

**問3の解答:**

```rust
//! 第11巻 UNIT 03 問3: 題に HTML の記号を入れたノートが、一覧で「ただの文字」として出るかを、
//! CSS セレクターで確かめる。
//! 実行: cargo test --test u03_q3 -- --nocapture

mod common;

use common::{body_text, test_app};
use scraper::{Html, Selector};

#[tokio::test]
async fn titles_are_text_not_html() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let title = r#"<b>太字</b> & "引用""#;
    app.create_note(&alice, title, "").await;

    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    let document = Html::parse_document(&html);
    let links: Vec<String> = document
        .select(&Selector::parse("ul.note-list li a").unwrap())
        .map(|link| link.text().collect())
        .collect();
    let bold = document
        .select(&Selector::parse("ul.note-list b").unwrap())
        .count();
    let raw_start = html.find("<li class=\"note-item\">").unwrap();
    let raw = &html[raw_start..raw_start + html[raw_start..].find("</a>").unwrap() + 4];

    println!("本文の中の形: {raw}");
    println!("リンクの文字: {links:?}");
    println!("一覧の中の b 要素: {bold} 個");
    assert_eq!(links, [title]);
    assert_eq!(bold, 0);
}
```

```text
本文の中の形: <li class="note-item"><a href="/notes/1">&lt;b&gt;太字&lt;/b&gt; &amp; "引用"</a>
リンクの文字: ["<b>太字</b> & \"引用\""]
一覧の中の b 要素: 0 個
```

- 本文の中では、`<` が `&lt;`、`>` が `&gt;`、`&` が `&amp;` に書き換えられていました（Topcoat のエスケープ。第6巻）。`"` は、要素の中の文字なので、そのままです。
- `scraper` で読むと、リンクの文字は、入力した題と**まったく同じ**に戻り、`b` 要素は 0 個でした。「HTML としてではなく、ただの文字として表示された」ことを、文字の並びの細かい形に頼らずに確かめられます。

### まとめ

- HTTP のテストでは、状態コード・ヘッダー（Location）・Set-Cookie（属性まで）・本文の4つを確かめる。まず全部を表示して、本当に返っているもの（と、返っていないもの）を見る。RustNote は、守りのヘッダーを返していない。
- 確かめの道具には `#[track_caller]` を付ける。付けると、失敗の場所が、道具の中ではなく、テストの呼んだ行になる。本文を読む `body_text` はレスポンスを受け取るので、状態コードは先に取っておく（E0382）。
- 同じ名前のヘッダーは `get_all` で全部読み、名前で探す。`Set-Cookie` の順番は決まっていない（cookie 0.18.2 の `HashSet`）ので、1つ目を読むテストは、ときどきだけ失敗する。本文は CSS セレクターで要素を決めて確かめる。`contains` は、検索の欄に戻した入力にも当たって、空の一覧でも通ってしまう。
