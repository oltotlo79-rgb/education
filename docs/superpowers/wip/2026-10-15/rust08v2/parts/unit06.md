## UNIT 06: クッキー・CSRF・Origin・SameSite

### この単元のゴール

- Topcoat の OriginPolicy が、どのリクエストを通し、どれを 403 で止めるかを、`Origin` と `Sec-Fetch-Site` の組み合わせの表で確かめ、その判断の順番を説明できる
- 本物のブラウザ（Chrome）で、`HttpOnly`（JavaScript から読めない）と `SameSite=Lax`（ほかのサイトからの送信にクッキーを付けない）が、実際にどう働くかを確かめられる
- CSRF の守りが、ブラウザの `SameSite` とサーバーの OriginPolicy の**2つの層**になっている理由と、片方を外したときに何が通るかを言える
- ほかのサービスからの POST を受け付けるときの `trust_origins` の書き方と、`cookie!` の書き間違いのエラーを読める

### たとえ話でつかむ

会社の、社内の書類の受付を考えます。

- 書類の封筒には、**どこから来たか**の印（差出人の部署）が押してあります。受付の人は、「経理部への支払いの依頼」が、社内の部署からではなく**外の会社の封筒**で来たら、受け取りません。中に社員の名前が書いてあっても、外から来た支払いの依頼は、だまされている心配があるからです。
- 社員証（クッキー）は、首から下げて、**社内でだけ**見せます。外のお店で「社員証を見せてください」と言われても、見せません。だから、外のお店から会社に電話をかけさせられても、社員証の無い電話になります。
- 「見るだけ」の用事（資料の閲覧）は、外から来てもかまいません。何も変わらないからです。変える用事（支払い・削除）だけを、厳しく確かめます。
- 取引先の銀行から来る「振り込みが終わりました」の書類は、決まった差出人だけ、例外として受け付けます。差出人の名前は、1文字も間違えずに登録します。

| 社内の書類の受付 | RustNote と Topcoat |
|---|---|
| 封筒の差出人の印 | `Origin`・`Sec-Fetch-Site` ヘッダー（ブラウザが付ける） |
| 外から来た「変える依頼」は受け取らない | OriginPolicy：GET 以外で、ほかのサイトからなら 403 |
| 社員証は社内でだけ見せる | クッキーの `SameSite=Lax`（ほかのサイトからの送信には付けない） |
| 見るだけの用事は通す | GET・HEAD・OPTIONS は確かめない |
| 取引先の銀行だけ例外 | `OriginPolicy::new().trust_origins([...])` |

> **用語:** CSRF（シーサーフ。Cross-Site Request Forgery：ほかのサイトのページが、ログイン中の利用者のブラウザに、本人の知らないうちにリクエストを送らせる攻撃。ブラウザはクッキーを付けて送るので、サーバーには本人からのリクエストに見える。第6巻 UNIT 10）

<!-- -->

> **用語:** サイト（ブラウザの決まりでの「同じサイト」は、`https` などの方式と、登録した名前（`example.com` の部分）が同じこと。`notes.example.com` と `evil.example.com` は、同じサイト（same-site）だが、違うオリジン（cross-origin）。`localhost` と `127.0.0.1` は、別のサイト）

<!-- -->

> **用語:** `Sec-Fetch-Site`（ブラウザが付ける、「このリクエストは、どこから来たか」の印。`same-origin`（同じオリジン）・`same-site`・`cross-site`・`none`（アドレス欄に打った、ブックマークなど）の4つ。ページの JavaScript からは変えられない）

### 図で理解する

OriginPolicy の判断の順番です（topcoat-router の `origin.rs` の `check`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="14" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="39" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① GET・HEAD・OPTIONS か</text>
<rect x="10" y="74" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="99" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② 外した道（exempt_paths）・信じる送り元か</text>
<rect x="10" y="134" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="159" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">③ Sec-Fetch-Site がある</text>
<rect x="10" y="194" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="219" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">④ Origin がある</text>
<rect x="10" y="254" width="300" height="40" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="160" y="279" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">⑤ どちらも無い（curl などのブラウザ以外）</text>
<line x1="160" y1="54" x2="160" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="160" y1="114" x2="160" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="160" y1="174" x2="160" y2="190" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="160" y1="234" x2="160" y2="250" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<text x="175" y="66" font-size="10" fill="#475569">いいえ</text>
<text x="175" y="126" font-size="10" fill="#475569">いいえ</text>
<text x="175" y="186" font-size="10" fill="#475569">無い</text>
<text x="175" y="246" font-size="10" fill="#475569">無い</text>
<line x1="310" y1="34" x2="396" y2="34" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="310" y1="94" x2="396" y2="94" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="310" y1="154" x2="396" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="310" y1="214" x2="396" y2="214" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<line x1="310" y1="274" x2="396" y2="274" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0601)"/>
<text x="406" y="38" font-size="12" font-weight="700" fill="#047857">通す（見るだけなので）</text>
<text x="406" y="98" font-size="12" font-weight="700" fill="#047857">通す</text>
<text x="406" y="150" font-size="12" fill="#0f172a">same-origin・none → <tspan font-weight="700" fill="#047857">通す</tspan></text>
<text x="406" y="168" font-size="12" fill="#0f172a">same-site・cross-site → <tspan font-weight="700" fill="#b91c1c">403</tspan></text>
<text x="406" y="210" font-size="12" fill="#0f172a">Host と同じ → <tspan font-weight="700" fill="#047857">通す</tspan></text>
<text x="406" y="228" font-size="12" fill="#0f172a">違う・null → <tspan font-weight="700" fill="#b91c1c">403</tspan></text>
<text x="406" y="278" font-size="12" font-weight="700" fill="#047857">通す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: OriginPolicy の判断の順番。見るだけのメソッドは確かめず、ブラウザが付ける Sec-Fetch-Site があればそれで、無ければ Origin と Host を比べて決める。どちらも無いリクエストは、ブラウザ以外からとして通す</div>
</div>

CSRF の守りの、2つの層です（例6-2で、本物のブラウザで確かめた結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="240" height="36" rx="6" fill="#0f172a"/>
<text x="130" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">ほかのサイトのページから</text>
<rect x="260" y="10" width="220" height="36" rx="6" fill="#0f172a"/>
<text x="370" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">ブラウザ：SameSite=Lax</text>
<rect x="490" y="10" width="220" height="36" rx="6" fill="#0f172a"/>
<text x="600" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">サーバー：OriginPolicy</text>
<text x="20" y="80" font-size="12" fill="#0f172a">fetch で GET（裏で読む）</text>
<text x="370" y="80" font-size="12" fill="#b91c1c" text-anchor="middle">クッキーを付けない</text>
<text x="600" y="80" font-size="12" fill="#475569" text-anchor="middle">GET なので通す</text>
<text x="20" y="126" font-size="12" fill="#0f172a">フォームで POST</text>
<text x="370" y="126" font-size="12" fill="#b91c1c" text-anchor="middle">クッキーを付けない</text>
<text x="600" y="126" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">403（cross-site）</text>
<text x="20" y="172" font-size="12" fill="#0f172a">リンクを開く（画面ごと GET）</text>
<text x="370" y="172" font-size="12" fill="#047857" text-anchor="middle">クッキーを付ける</text>
<text x="600" y="172" font-size="12" fill="#475569" text-anchor="middle">GET なので通す（変えない）</text>
<text x="20" y="218" font-size="12" fill="#0f172a">同じサイトの別の名前から POST</text>
<text x="370" y="218" font-size="12" fill="#047857" text-anchor="middle">クッキーを付ける（same-site）</text>
<text x="600" y="218" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">403（same-site）</text>
<line x1="10" y1="102" x2="710" y2="102" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="148" x2="710" y2="148" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="194" x2="710" y2="194" stroke="#e2e8f0" stroke-width="1"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: CSRF の2つの層。SameSite=Lax はほかのサイトからの POST や裏の読み込みにクッキーを付けず、OriginPolicy は GET 以外のほかのサイトからのリクエストを 403 にする。同じサイトの別の名前からは、OriginPolicy だけが止める</div>
</div>

### まず動かす最小コード

#### 例6-1: OriginPolicy の判断を、表にして確かめる

RustNote は、OriginPolicy の設定を何も書いていません。`build_router` の `module_router!()…build()` が、Topcoat の決まりの `OriginPolicy::new()`（信じる送り元なし・外す道なし）を使います。

> **▼ このコードがやること（先に日本語で）:** alice を登録してから、ノートを作る `/notes` に、10 通りのリクエストを送ります。どれも alice のクッキーと `Host: 127.0.0.1:3000` を付け、`Origin` と `Sec-Fetch-Site` の付け方だけを変えます（9つは POST、最後の1つは GET）。返事の状態コードと、ノートが増えたかを表示します。

```rust
//! 第8巻 UNIT 06: Topcoat の OriginPolicy が、どのリクエストを通し、どれを 403 で止めるかを、表にして確かめる。
//! どれも alice のクッキーを付けて送る（ブラウザが付けて送った、と考える）。
//! 実行: cargo test --test u06_policy -- --nocapture

mod common;

use common::{TestApp, encode, test_app};
use topcoat::router::{Body, Method, header, request::Request};

async fn notes(app: &TestApp) -> i64 {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await.unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

/// 1つのやり方: (説明, メソッド, 付けるヘッダー)
type Case<'a> = (&'a str, Method, &'a [(&'a str, &'a str)]);

#[tokio::test]
async fn origin_policy_table() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let form = format!("title={}&body=", encode("牛乳を買う"));
    let cases: [Case; 10] = [
        ("ヘッダーが何も無い（curl など）", Method::POST, &[]),
        (
            "Origin が同じ",
            Method::POST,
            &[("origin", "http://127.0.0.1:3000")],
        ),
        (
            "Origin がほかのサイト",
            Method::POST,
            &[("origin", "https://evil.example")],
        ),
        ("Origin が null", Method::POST, &[("origin", "null")]),
        (
            "Sec-Fetch-Site: same-origin",
            Method::POST,
            &[("sec-fetch-site", "same-origin")],
        ),
        (
            "Sec-Fetch-Site: none",
            Method::POST,
            &[("sec-fetch-site", "none")],
        ),
        (
            "Sec-Fetch-Site: same-site",
            Method::POST,
            &[("sec-fetch-site", "same-site")],
        ),
        (
            "Sec-Fetch-Site: cross-site",
            Method::POST,
            &[("sec-fetch-site", "cross-site")],
        ),
        (
            "same-origin と、ほかのサイトの Origin",
            Method::POST,
            &[
                ("sec-fetch-site", "same-origin"),
                ("origin", "https://evil.example"),
            ],
        ),
        (
            "GET で cross-site",
            Method::GET,
            &[("sec-fetch-site", "cross-site")],
        ),
    ];
    for (n, (what, method, extra)) in cases.into_iter().enumerate() {
        let before = notes(&app).await;
        let mut builder = Request::builder()
            .method(method.clone())
            .uri("/notes")
            .header(header::HOST, "127.0.0.1:3000")
            .header(header::COOKIE, alice.as_str());
        if method == Method::POST {
            builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }
        for (name, value) in extra {
            builder = builder.header(*name, *value);
        }
        let body = if method == Method::POST {
            Body::from(form.clone())
        } else {
            Body::empty()
        };
        let response = app.router.handle(builder.body(body).unwrap()).await;
        let made = notes(&app).await - before;
        println!(
            "{:>2}. {method} {what} → {}（ノート +{made}）",
            n + 1,
            response.status()
        );
    }
}
```

```powershell
cargo test --test u06_policy -- --nocapture
```

```text
 1. POST ヘッダーが何も無い（curl など） → 303 See Other（ノート +1）
 2. POST Origin が同じ → 303 See Other（ノート +1）
 3. POST Origin がほかのサイト → 403 Forbidden（ノート +0）
 4. POST Origin が null → 403 Forbidden（ノート +0）
 5. POST Sec-Fetch-Site: same-origin → 303 See Other（ノート +1）
 6. POST Sec-Fetch-Site: none → 303 See Other（ノート +1）
 7. POST Sec-Fetch-Site: same-site → 403 Forbidden（ノート +0）
 8. POST Sec-Fetch-Site: cross-site → 403 Forbidden（ノート +0）
 9. POST same-origin と、ほかのサイトの Origin → 303 See Other（ノート +1）
10. GET GET で cross-site → 200 OK（ノート +0）
```

（結果の行だけを抜き出しています）

#### ▼ コードを1つずつ分解して解説

##### 解説1: Sec-Fetch-Site があれば、それで決まる

- 5の `same-origin` と6の `none` は通り、7の `same-site` と8の `cross-site` は 403 でした。`same-site`（同じサイトの、別の名前のページから）も止めるのは、ふつうよりも厳しい決め方です。
- 9は、`Sec-Fetch-Site: same-origin` と、ほかのサイトの `Origin` を両方付けたもので、**通りました**。`Sec-Fetch-Site` があると、`Origin` は見ません。`Sec-Fetch-Site` は、ブラウザだけが付けられる印で、ページの JavaScript からは変えられないので、ブラウザから来たリクエストなら、正しいと信じてよいからです。
- このテストでは、ヘッダーを自分で付けています。本物の攻撃は、ブラウザに送らせるものなので、9のような組み合わせは、ふつうのブラウザからは来ません。

##### 解説2: Sec-Fetch-Site が無ければ、Origin と Host を比べる

- 2の `Origin: http://127.0.0.1:3000` は、`Host: 127.0.0.1:3000` と同じなので通り、3のほかのサイトは 403 でした。4の `Origin: null` も 403 です。`null` は、ファイルを直接開いたページや、制限をかけた枠（`sandbox` の `iframe`）から送られたときに付く値で、どこから来たか分からないので、止めます。
- 1のどちらも無いリクエスト（curl やテストの道具など）は、通りました。CSRF は「ブラウザに送らせる」攻撃なので、ブラウザ以外の道具からのリクエストは、ここでは止めません（ログインのクッキーを持っているのは、その道具を使う本人だからです）。
- 10の GET は、`cross-site` でも `200 OK` でした。GET は「見るだけ」の約束なので、確かめません。その代わり、**GET で何かを変えてはいけない**のです（第10巻 UNIT 06）。

### もう一歩: 実務に近い形

#### 例6-2: 本物のブラウザで、HttpOnly と SameSite を確かめる

クッキーの `HttpOnly` と `SameSite` は、ブラウザが守る決まりなので、テストの道具（ルーターに直接リクエストを渡す）では確かめられません。そこで、本物の Chrome で確かめます。

- RustNote を `http://127.0.0.1:3000` で起動し、alice でログインしておきます。
- **ほかのサイト**として、`http://localhost:8000` で、次の HTML を出します（Python の `python -m http.server 8000 --bind localhost` で、HTML のファイルを置いたフォルダを配りました）。`localhost` と `127.0.0.1` は、ブラウザにとって別のサイトです。

```html
<!doctype html>
<html lang="ja">
<meta charset="utf-8">
<title>ほかのサイト</title>
<h1>ほかのサイト（localhost:8000）</h1>
<form id="f" method="post" action="http://127.0.0.1:3000/notes">
  <input name="title" value="ほかのサイトから作ったノート">
  <input name="body" value="">
  <button id="send">送る</button>
</form>
<p><a id="link" href="http://127.0.0.1:3000/notes">RustNote の一覧へのリンク</a></p>
```

**1. HttpOnly:** RustNote の画面（ログイン中）の Console で、`document.cookie` を読みました。

```text
（127.0.0.1:3000 で alice としてログインしたあと、RustNote の画面の Console で）
> ({ "document.cookie の長さ": document.cookie.length, "空": document.cookie === "" })
{
  "document.cookie の長さ": 0,
  "空": true
}
```

- ログイン中なのに、`document.cookie` は空（長さ 0）でした。セッションの `__Host-session` も、お知らせの `flash` も、`HttpOnly` が付いているので、ページの JavaScript からは見えません。もし誰かの書いたプログラムが RustNote の画面で動いてしまっても（XSS）、セッションのトークンを盗み出すことはできません。

**2. ほかのサイトから、裏で読み込む:** `localhost:8000` のページの Console で、RustNote の一覧を `fetch` で読みました（`credentials: 'include'` は「クッキーも付けて」という頼み）。

```text
（localhost:8000 のページの Console で）
> const r = await fetch('http://127.0.0.1:3000/notes', { credentials: 'include', mode: 'no-cors' }); ({ type: r.type, status: r.status, page: location.href })
{
  "page": "http://localhost:8000/csrf.html",
  "status": 0,
  "type": "opaque"
}
（RustNote の窓のアクセスログ）
GET /notes -> 303 See Other (292.6µs)
GET /login -> 200 OK (338µs)
```

- RustNote の窓のアクセスログでは、`GET /notes -> 303 See Other`（ログイン画面へ）でした。ブラウザが、`SameSite=Lax` のクッキーを**付けなかった**ので、RustNote には「ログインしていない人」に見えたのです。ページの側は、中身を読めない `opaque`（不透明）の返事しか受け取れません。

**3. ほかのサイトのフォームで POST:** `localhost:8000` のページの「送る」を押しました。

```text
（localhost:8000 のページで「送る」を押したあと）
{
  "text": "forbidden",
  "url": "http://127.0.0.1:3000/notes"
}
（RustNote の窓のアクセスログ: この POST の行は無い）
```

- 画面には `forbidden`（403）が出て、ノートは作られませんでした。RustNote のアクセスログにも、この POST の行はありません。OriginPolicy が、RustNote のページの処理（とアクセスログの層）より前で止めたからです（UNIT 01）。このとき、ブラウザは `SameSite=Lax` のクッキーを付けないので、仮に OriginPolicy が無くても、ログインしていない人の POST になります。**2つの層**が重なっているのです。

**4. ほかのサイトのリンクを開く:** `localhost:8000` のページの「RustNote の一覧へのリンク」を押しました。

```text
（localhost:8000 のページで「RustNote の一覧へのリンク」を押したあと）
{
  "h1": "ノート",
  "notes": [],
  "url": "http://127.0.0.1:3000/notes",
  "user": "alice@example.com"
}
（RustNote の窓のアクセスログ）
GET /notes -> 200 OK (6.5559ms)
```

- 今度は `GET /notes -> 200 OK` で、右上は `alice@example.com`、つまり**クッキーが付いて**いました。`SameSite=Lax` は、画面ごと移る GET（リンクを開く）には、クッキーを付けます。ほかのサイトから RustNote へのリンクを開いたとき、毎回ログインし直しにならないためです。だから、GET で何かを変える画面があると、ほかのサイトのリンクで動いてしまいます（第10巻 UNIT 06 のエラー1）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: SameSite=Lax があるからと、OriginPolicy を外した

`build_router` に `.origin_policy(OriginPolicy::dangerous_disable())` を足して、Origin の確かめを外した場合です（練習用のコピーの `src/routes.rs` を書き換え、確かめたら元に戻します）。例6-1のテストを動かします。

```text
 1. POST ヘッダーが何も無い（curl など） → 303 See Other（ノート +1）
 2. POST Origin が同じ → 303 See Other（ノート +1）
 3. POST Origin がほかのサイト → 303 See Other（ノート +1）
 4. POST Origin が null → 303 See Other（ノート +1）
 5. POST Sec-Fetch-Site: same-origin → 303 See Other（ノート +1）
 6. POST Sec-Fetch-Site: none → 303 See Other（ノート +1）
 7. POST Sec-Fetch-Site: same-site → 303 See Other（ノート +1）
 8. POST Sec-Fetch-Site: cross-site → 303 See Other（ノート +1）
 9. POST same-origin と、ほかのサイトの Origin → 303 See Other（ノート +1）
10. GET GET で cross-site → 200 OK（ノート +0）
```

- **何が起きたか:** 9つの POST が、全部 `303` で通り、ノートが作られました。3・4・7・8の、ほかのサイトからの POST もです。
- **SameSite だけでは足りない理由:** 7の `same-site` は、ブラウザにとって「同じサイト」なので、`SameSite=Lax` のクッキーは**付いて送られます**（図の2つ目の最後の行）。たとえば、利用者が自由にページを作れる `pages.example.com` から、`notes.example.com` の RustNote に POST を送らせると、クッキー付きで届きます。これを止めるのは、OriginPolicy だけです。また、古いブラウザには `SameSite` の決まりが無いものもあります。
- **学べること:** `dangerous_disable`（危ない、外す）という名前のとおり、外すのは、アプリが自分で別の守り（CSRF トークンなど）を持っているときだけです。

#### エラー2（エラーが出ない間違い）: 信じる送り元の最後に / を付けた

決済のサービス（`https://pay.example`）のページから、支払いが終わったあとに RustNote へ POST で戻ってくる、という作りにしたい場合です。`trust_origins(["https://pay.example/"])` と、最後に `/` を付けて書きました。決済のサービスからの POST を確かめるテスト（`tests/u06_trust.rs`）を動かします。

```rust
//! 第8巻 UNIT 06: 決済サービス（https://pay.example）のページから、RustNote へ POST で戻ってくる場合。
//! ブラウザは Origin と Sec-Fetch-Site を付けて送る。
//! 実行: cargo test --test u06_trust -- --nocapture

mod common;

use common::{encode, test_app};
use topcoat::router::{Body, Method, header, request::Request};

#[tokio::test]
async fn post_back_from_a_partner() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for origin in ["https://pay.example", "https://evil.example"] {
        let request = Request::builder()
            .method(Method::POST)
            .uri("/notes")
            .header(header::HOST, "127.0.0.1:3000")
            .header(header::COOKIE, alice.as_str())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header("origin", origin)
            .header("sec-fetch-site", "cross-site")
            .body(Body::from(format!(
                "title={}&body=",
                encode("支払いのメモ")
            )))
            .unwrap();
        let status = app.router.handle(request).await.status();
        println!("Origin: {origin} → {status}");
    }
}
```

RustNote のコード（何も信じていない）のときです。

```text
Origin: https://pay.example → 403 Forbidden
Origin: https://evil.example → 403 Forbidden
```

`/` を付けて書いた版です。

```text
Origin: https://pay.example → 403 Forbidden
Origin: https://evil.example → 403 Forbidden
```

`/` を付けずに `trust_origins(["https://pay.example"])` と書いた版です。

```text
Origin: https://pay.example → 303 See Other
Origin: https://evil.example → 403 Forbidden
```

- **何が起きたか:** `/` を付けた版では、決済のサービスからの POST も 403 のままでした。ブラウザが付ける `Origin` は `https://pay.example`（最後の `/` が無い）で、Topcoat は、この文字列とそのまま比べるからです。`/` を付けない版では、決済のサービスだけが `303` で通り、ほかのサイトは 403 のままでした。
- **学べること:** `Origin` は「方式・名前・（決まりと違えば）ポート番号」だけの文字列で、最後に `/` も、道（`/pay` など）も付きません。設定したのに効かないときは、本物の `Origin` を確かめてから書きます。
- 信じる送り元を足すと、そこからは、ログイン中の利用者のクッキー付きの POST が通るようになります。決済のサービスのように、本当に信じてよい所だけにします。

#### エラー3: SameSite の値を書き間違えた（E0425）

お知らせのクッキー（`src/flash.rs` の `set`）の `SameSite = Lax;` を、`SameSite = Loose;` と書いた場合です。

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0425]: cannot find value `Loose` in this scope
  --> src\flash.rs:92:20
   |
92 |         SameSite = Loose;
   |                    ^^^^^ not found in this scope

```

- **何が起きたか:** 「`Loose` という値が見つかりません」というエラーです。`cookie!` マクロの `SameSite` には、`Strict`・`Lax`・`None` のどれかを書きます。マクロが、書いた名前をそのまま値として探すので、決まりに無い名前は、コンパイルで止まります。
- `Strict` にすると、ほかのサイトのリンクから開いたときにも、クッキーを付けません（例6-2の4が、ログインしていない画面になります）。`None` にすると、ほかのサイトからの POST にもクッキーを付けるので、OriginPolicy だけが頼りになります（`None` のときは `Secure` も必要です）。

### RustNote ではここで使う

- OriginPolicy は、RustNote のコードには1行も書かれていません。Topcoat のルーターが、いちばん外側の層として、決まりの `OriginPolicy::new()` を必ず付けるからです。外すには、`dangerous_disable` と、わざわざ書く必要があります。
- セッションのクッキー（`__Host-session`）の属性は、Topcoat が決めています（`HttpOnly; SameSite=Lax; Secure; Path=/`）。`__Host-` という名前の始まりは、「`Secure` で、`Path=/` で、`Domain` を付けない」ときだけブラウザが受け付ける、という決まりで、同じサイトのほかの名前（`evil.example.com`）からは、このクッキーを書き換えられません。
- お知らせのクッキー（`flash`）は、RustNote が `cookie!` で作っていて、`Path=/`・`HttpOnly`・`SameSite=Lax`・`Max-Age=5 分` です（第10巻 UNIT 09）。中身は決まったコードだけなので、`Secure` は付けていません。
- 状態を変える操作は、全部 POST です（作成・保存・完了・削除・ログイン・ログアウト・登録）。GET で変える画面はありません。

### 練習問題

**問1:** 例6-1の1（ヘッダーが何も無い POST）は、通りました。攻撃する人が、curl で alice のクッキーを付けて POST を送れば、通ってしまうのではないでしょうか。

**問2:** 例6-2の2で、ほかのサイトのページの `fetch` は、RustNote の返事を読めたでしょうか。読めなかったのは、なぜでしょうか。

**問3:** 「ログアウトだけは GET でもよいのでは」と言われました。ほかのサイトのリンクで、勝手にログアウトさせられると、何が困るでしょうか。

### 解答と解説

**問1の解答:**

- 通ります。ただし、curl で送るには、攻撃する人が **alice のクッキーそのもの**を持っていなければなりません。クッキーを持っていれば、CSRF をしなくても、alice として何でもできます（UNIT 05 のエラー1の「写し取ったクッキー」）。CSRF は、クッキーを持っていない攻撃する人が、alice の**ブラウザに**送らせる攻撃なので、ブラウザが付ける `Origin` や `Sec-Fetch-Site` で見分けられます。
- だから、OriginPolicy は「ブラウザから、ほかのサイトの印付きで来た、変えるリクエスト」を止め、印の無いリクエストは通します。クッキーそのものを盗まれないための守りは、`HttpOnly` と `Secure`（例6-2の1・UNIT 01）です。

**問2の解答:**

- 読めませんでした。返事は `type: "opaque"`・`status: 0` で、中身も状態コードも見えません。ほかのサイトのページから、RustNote の返事を読むには、RustNote が「このサイトには読ませてよい」という返事のヘッダー（CORS の `Access-Control-Allow-Origin` など）を付ける必要があり、RustNote は付けていないからです。
- しかも、例6-2の2のとおり、クッキーも付いていませんでした。もし読めたとしても、中身は「ログインしていない人」へのログイン画面です。

**問3の解答:**

- ほかのサイトに、RustNote の `/logout` へのリンクを置いておき、利用者に押させるだけで、勝手にログアウトさせられます。リンクを開く GET には、`SameSite=Lax` でもクッキーが付き（例6-2の4）、OriginPolicy も GET は確かめないからです。データは消えませんが、利用者は、なぜかログアウトされる、を繰り返すことになり、ログインのし直しを狙った、だましの画面（偽のログイン画面）に誘い込む手口にも使えます。
- RustNote のログアウトは、`src/routes/logout.rs` のコメントのとおり、「状態を変える操作なので、GET ではなく POST」にしています。

### まとめ

- OriginPolicy は、GET・HEAD・OPTIONS を通し、それ以外は、外した道・信じる送り元 → `Sec-Fetch-Site`（same-origin・none だけ通す）→ `Origin` と `Host` の比べ → どちらも無ければ通す、の順に決める。ほかのサイトからも、同じサイトの別の名前からも、変えるリクエストは 403。
- 本物のブラウザで、`HttpOnly` のクッキーは `document.cookie` に出ず、`SameSite=Lax` のクッキーは、ほかのサイトからの `fetch` や POST には付かず、リンクを開く GET には付いた。CSRF の守りは、ブラウザの SameSite とサーバーの OriginPolicy の2つの層で、OriginPolicy を外すと、同じサイトの別の名前からの POST が通る。
- ほかのサービスからの POST は `trust_origins` で信じる。値は `Origin` と同じ文字列（最後に `/` を付けない）。状態を変える操作は、ログアウトも含めて、全部 POST にする。
