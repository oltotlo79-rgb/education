## UNIT 10: 守りのテスト・監査・事故の対応

### この単元のゴール

- この巻で確かめた守り（認可・ログインの失敗・セッション・ログアウト・CSRF）を1本のテストにまとめ、守りを1つ外すと、**その守りの名前で**テストが止まることを確かめられる
- 今の RustNote のアクセスログに、何が残り、何が残らないか（誰が・どこから・Origin の検査の 403）を、実際の出力で言える
- ログインとログアウトの**監査の記録**を足し、書くこと（いつ・どこから・誰が・何をして・どうなったか）と、書かないこと（パスワード・トークン）を説明できる
- 「DB が漏れたかもしれない」ときに、漏れたハッシュでは入れないことと、全員のセッションを一度に終わらせる手順を、テストで確かめられる

### たとえ話でつかむ

お店の防犯を考えます。

- **毎晩の戸締まりの点検表:** 「裏口の鍵・金庫・窓」のように、守る所を1枚に並べ、毎晩同じ順に確かめます。どれかが開いていたら、**どの行で止まったか**で、すぐ分かります。これが**守りのテスト**です。
- **防犯カメラの録画:** 何かあったときに、「いつ・どの入口から・誰が・何をしたか」をあとから見られるようにします。ただし、お客さんが打った**暗証番号の手元まで**は映しません。これが**監査の記録**です。
- **事故のときの手順書:** 「合鍵が盗まれたかもしれない」と分かったら、まず錠前を全部替え、録画を見て、お客さんに知らせ、原因を直し、点検表に1行足します。慌てないように、順番を先に決めておきます。これが**事故の対応**です。

| お店の防犯 | RustNote の守り |
|---|---|
| 戸締まりの点検表 | 守りのテスト（例10-1） |
| 点検表の行の名前 | `assert_eq!` の3つ目に書いた、守りの名前 |
| 防犯カメラの録画 | 監査の記録（例10-2・例10-3） |
| 暗証番号の手元は映さない | パスワード・トークン・クッキーの値は書かない |
| 錠前を全部替える | `DELETE FROM sessions`（全員のセッションを終わらせる、例10-4） |
| 手順書に1行足す | 事故から分かった守りを、守りのテストに足す |

> **用語:** 回帰テスト（一度直した間違いや、一度作った守りが、あとの変更で元に戻って（回帰して）いないかを、毎回確かめるテスト。この単元の守りのテストも、その1つ）

<!-- -->

> **用語:** 監査の記録（監査ログ：ログインの成功・失敗・ログアウトのような、守りに関わる出来事を、あとから調べられるように1件ずつ残す記録。アクセスログより、「誰が」「どうなったか」をはっきり書く）

<!-- -->

> **用語:** 事故の対応（インシデント対応：秘密が漏れた・乗っ取られた、のような、利用者に影響が出る出来事に気づいてから、止める・調べる・知らせる・直す・振り返るまでの動き。第7巻 UNIT 10 では、DB のトークンが漏れたときを扱った）

### 図で理解する

3つの道具と、使う時です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-1001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="210" height="250" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="115" y="46" font-size="12" fill="#047857" text-anchor="middle">出す前に、毎回</text>
<text x="115" y="70" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">守りのテスト</text>
<text x="115" y="90" font-size="11" fill="#047857" text-anchor="middle">（例10-1）</text>
<text x="26" y="124" font-size="11" fill="#0f172a">・この巻の守りを、1本に</text>
<text x="26" y="150" font-size="11" fill="#0f172a">・守り1つに、名前を1つ</text>
<text x="26" y="176" font-size="11" fill="#0f172a">・外れたら、その名前で止まる</text>
<text x="26" y="202" font-size="11" fill="#0f172a">・第11巻 UNIT 06 で、</text>
<text x="38" y="222" font-size="11" fill="#0f172a">もっと増やして毎回回す</text>
<rect x="255" y="20" width="210" height="250" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="46" font-size="12" fill="#1d4ed8" text-anchor="middle">動いている間</text>
<text x="360" y="70" font-size="14" font-weight="700" fill="#1d4ed8" text-anchor="middle">監査の記録</text>
<text x="360" y="90" font-size="11" fill="#1d4ed8" text-anchor="middle">（例10-2・例10-3）</text>
<text x="271" y="124" font-size="11" fill="#0f172a">・いつ・どこから・誰が</text>
<text x="271" y="150" font-size="11" fill="#0f172a">・何をして・どうなったか</text>
<text x="271" y="176" font-size="11" fill="#0f172a">・パスワード・トークンは</text>
<text x="283" y="196" font-size="11" fill="#0f172a">書かない</text>
<text x="271" y="222" font-size="11" fill="#b91c1c">・Origin の検査の 403 は</text>
<text x="283" y="242" font-size="11" fill="#b91c1c">アプリの中に残らない</text>
<rect x="500" y="20" width="210" height="250" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="605" y="46" font-size="12" fill="#c2410c" text-anchor="middle">事故のあと</text>
<text x="605" y="70" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">事故の対応</text>
<text x="605" y="90" font-size="11" fill="#c2410c" text-anchor="middle">（例10-4）</text>
<text x="516" y="124" font-size="11" fill="#0f172a">・止める: セッションを</text>
<text x="528" y="144" font-size="11" fill="#0f172a">全部消す</text>
<text x="516" y="170" font-size="11" fill="#0f172a">・調べる: 監査の記録を読む</text>
<text x="516" y="196" font-size="11" fill="#0f172a">・知らせる・パスワードを</text>
<text x="528" y="216" font-size="11" fill="#0f172a">変えてもらう</text>
<text x="516" y="242" font-size="11" fill="#0f172a">・直す・振り返る</text>
<line x1="222" y1="145" x2="251" y2="145" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-1001)"/>
<line x1="467" y1="145" x2="496" y2="145" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-1001)"/>
<path d="M605,272 L605,298 L115,298 L115,276" fill="none" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-08-1001)"/>
<text x="360" y="320" font-size="11" fill="#0f172a" text-anchor="middle">事故から分かった守りを、守りのテストに1つ足す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 守りのテストは出す前に、監査の記録は動いている間に、事故の対応は事故のあとに使う。事故から分かったことは、守りのテストに戻す</div>
</div>

監査の記録を、リクエストの通り道のどこで書くかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-1002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="90" height="50" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="55" y="70" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">リクエスト</text>
<rect x="130" y="30" width="160" height="70" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="210" y="58" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">Origin の検査</text>
<text x="210" y="80" font-size="11" fill="#b91c1c" text-anchor="middle">（Topcoat・一番外側）</text>
<rect x="320" y="30" width="160" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="400" y="58" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">access_log</text>
<text x="400" y="80" font-size="11" fill="#1d4ed8" text-anchor="middle">（routes.rs の層）</text>
<rect x="510" y="30" width="200" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="610" y="58" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ページ</text>
<text x="610" y="80" font-size="11" fill="#047857" text-anchor="middle">（login.rs・logout.rs）</text>
<line x1="102" y1="65" x2="126" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-1002)"/>
<line x1="292" y1="65" x2="316" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-1002)"/>
<line x1="482" y1="65" x2="506" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-1002)"/>
<line x1="210" y1="102" x2="210" y2="136" stroke="#ef4444" stroke-width="2" marker-end="url(#ar-08-1002)"/>
<text x="210" y="156" font-size="11" fill="#b91c1c" text-anchor="middle">断ると、すぐ 403</text>
<text x="210" y="176" font-size="11" fill="#b91c1c" text-anchor="middle">内側の層もページも動かない</text>
<text x="210" y="196" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">→ どの記録にも残らない</text>
<text x="400" y="138" font-size="11" fill="#0f172a" text-anchor="middle">今の RustNote にある記録</text>
<text x="400" y="160" font-size="10" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">POST /login -> 401 ...</text>
<text x="400" y="182" font-size="11" fill="#0f172a" text-anchor="middle">何が起きたかは分かるが</text>
<text x="400" y="202" font-size="11" fill="#0f172a" text-anchor="middle">誰が・どこからは無い</text>
<text x="610" y="138" font-size="11" fill="#0f172a" text-anchor="middle">例10-3で足す記録</text>
<text x="610" y="160" font-size="10" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">[audit] ... login_failed</text>
<text x="610" y="182" font-size="11" fill="#0f172a" text-anchor="middle">結果が分かる所で書くので</text>
<text x="610" y="202" font-size="11" fill="#0f172a" text-anchor="middle">誰が・どうなったかが分かる</text>
<text x="360" y="244" font-size="11" fill="#475569" text-anchor="middle">どの記録にも、パスワード・トークン・クッキーの値は書かない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: リクエストの通り道と、記録が残る場所。Origin の検査が断った 403 は、アクセスログにも、ページで書く監査の記録にも残らない</div>
</div>

### まず動かす最小コード

#### 例10-1: この巻の守りを、1本のテストにまとめる

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、alice のノートを1つ作ります。そのあと、この巻で確かめた5つの守りを、順に確かめます。1つ確かめ終わるたびに `✓ 守りの名前` を表示します。どれかが外れていたら、`assert_eq!` の3つ目に書いた**守りの名前**を出して止まります。

```rust
//! 第8巻 UNIT 10: この巻で確かめた守りを、1本の「守りのテスト」にまとめる。
//! どれかの守りが外れたら、その守りの名前で止まる。
//! 実行: cargo test --test u10_guards -- --nocapture

mod common;

use common::{TestApp, body_text, encode, session_cookie, test_app};
use topcoat::router::{Body, Method, StatusCode, header, request::Request};

const PASSWORD: &str = "correct horse battery";

async fn login(app: &TestApp, email: &str, password: &str) -> topcoat::router::response::Response {
    let form = format!("email={}&password={}", encode(email), encode(password));
    app.post_form("/login", &form, None).await
}

fn ok(name: &str) {
    println!("✓ {name}");
}

#[tokio::test]
async fn guards() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;

    // UNIT 01・07: ログインしていなければ 303、他人のノートは、どの入口でも 404
    assert_eq!(
        app.get("/notes/1", None).await.status(),
        StatusCode::SEE_OTHER,
        "ログインしていない人は、ログイン画面へ"
    );
    for (path, form) in [
        ("/notes/1", None),
        ("/notes/1/edit", None),
        ("/notes/1/toggle", Some("")),
        ("/notes/1/delete", Some("")),
    ] {
        let response = match form {
            Some(form) => app.post_form(path, form, Some(&bob)).await,
            None => app.get(path, Some(&bob)).await,
        };
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "他人のノートは 404: {path}"
        );
    }
    ok("認可: ログインしていなければ 303、他人のノートはどの入口でも 404");

    // UNIT 04: ログインの失敗は、登録の有無で同じ画面
    let a = body_text(login(&app, "alice@example.com", "wrong password").await).await;
    let b = body_text(login(&app, "dave@example.com", "wrong password").await).await;
    assert_eq!(
        a.replace("alice@example.com", "…"),
        b.replace("dave@example.com", "…"),
        "ログインの失敗の画面は、登録の有無で同じ"
    );
    ok("ログイン: 登録の無いメールも、違うパスワードも、同じ画面");

    // UNIT 04: ログインのたびにトークンが新しくなり、前のトークンは使えない
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode(PASSWORD)
    );
    let renewed = session_cookie(&app.post_form("/login", &form, Some(&alice)).await).unwrap();
    assert_ne!(renewed, alice, "ログインのたびに、新しいトークン");
    assert_eq!(
        app.get("/notes", Some(&alice)).await.status(),
        StatusCode::SEE_OTHER,
        "前のトークンは使えない"
    );
    ok("セッション: ログインのたびに新しいトークン、前のトークンは使えない");

    // UNIT 05: ログアウトで DB の記録が消え、写し取ったクッキーも使えない
    let copied = renewed.clone();
    app.post_form("/logout", "", Some(&renewed)).await;
    assert_eq!(
        app.get("/notes", Some(&copied)).await.status(),
        StatusCode::SEE_OTHER,
        "ログアウトのあと、写し取ったクッキーは使えない"
    );
    ok("ログアウト: 写し取ったクッキーも使えない");

    // UNIT 06: ほかのサイトからの POST は 403
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, bob.as_str())
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("sec-fetch-site", "cross-site")
        .body(Body::from("title=x&body="))
        .unwrap();
    assert_eq!(
        app.router.handle(request).await.status(),
        StatusCode::FORBIDDEN,
        "ほかのサイトからの POST は 403"
    );
    ok("CSRF: ほかのサイトからの POST は 403");
}
```

```powershell
cargo test --test u10_guards -- --nocapture
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running tests\u10_guards.rs (target\debug\deps\u10_guards-ceff6f46bfcca925.exe)

running 1 test
POST /signup -> 303 See Other (482.2363ms)
POST /signup -> 303 See Other (474.2321ms)
POST /notes -> 303 See Other (36.4292ms)
GET /notes/1 -> 303 See Other (236.1µs)
GET /notes/1 -> 404 Not Found (6.3756ms)
GET /notes/1/edit -> 404 Not Found (4.3368ms)
POST /notes/1/toggle -> 404 Not Found (6.2ms)
POST /notes/1/delete -> 404 Not Found (3.4258ms)
✓ 認可: ログインしていなければ 303、他人のノートはどの入口でも 404
POST /login -> 401 Unauthorized (393.9211ms)
POST /login -> 401 Unauthorized (789.0738ms)
✓ ログイン: 登録の無いメールも、違うパスワードも、同じ画面
POST /login -> 303 See Other (438.2771ms)
GET /notes -> 303 See Other (2.1151ms)
✓ セッション: ログインのたびに新しいトークン、前のトークンは使えない
POST /logout -> 303 See Other (13.0876ms)
GET /notes -> 303 See Other (2.0733ms)
✓ ログアウト: 写し取ったクッキーも使えない
✓ CSRF: ほかのサイトからの POST は 403
test guards ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.70s
```

- 5つの `✓` が出て、テストが通りました。`✓` の前の行は、RustNote のアクセスログ（`routes.rs` の `access_log`）です。守りごとに、どのリクエストが送られ、どう返ったかが分かります。
- CSRF の守り（最後）だけ、アクセスログの行がありません。Topcoat の Origin の検査が、アクセスログの層より前で 403 を返したからです（UNIT 06）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 守り1つに、名前を1つ

- `assert_eq!(実際の値, 期待する値, "守りの名前")` の3つ目の引数は、外れたときにだけ表示されます。数字（`left: 200`・`right: 303`）だけでは、何の守りが外れたのか分かりませんが、名前があれば、すぐ分かります（エラー1）。
- 5つの守りは、この巻の UNIT と、次のように対応します。

| 守りの名前 | 確かめていること | UNIT |
|---|---|---|
| 認可 | ログインしていなければ 303、bob からは alice のノートが、4つの入口のどれでも 404 | 01・07 |
| ログイン | 登録の無いメール（`dave`）と、違うパスワード（`alice`）の失敗の画面が、メールの所を除いて同じ | 04 |
| セッション | ログインのたびにトークンが変わり、前のトークンは使えない | 04・05 |
| ログアウト | ログアウトのあと、写し取ったクッキーでは入れない | 05 |
| CSRF | ほかのサイトからの POST は 403 | 06 |

- 失敗の画面には、入力したメールアドレスが、フォームに戻して表示されます（`value=(email)`）。そこだけ `…` に置き換えてから比べるので、「メールの所を除いて同じ」を確かめられます。

##### 解説2: アクセスログの時間も、一緒に読む

- 「ログイン」の守りの2回目（`dave`）だけ、`401` まで約 0.8 秒かかっています（1回目の alice は約 0.4 秒）。登録の無いメールで照合する `DUMMY_HASH` を、このときに初めて作ったからです（`LazyLock`。UNIT 02 の問2・UNIT 04 で見た、起動して最初の1回だけ遅い改善点）。
- 守りのテストが通っても、アクセスログを一緒に読むと、こういう小さな違いに気づけます。

### もう一歩: 実務に近い形

#### 例10-2: 今の RustNote が残す記録を見る

> **▼ このコードがやること（先に日本語で）:** 攻撃する人（`203.0.113.7`）と alice（`198.51.100.20`）のリクエストを、5つの場面に分けて順に送ります。どちらも、説明用に決められたアドレスです。本物のサーバーでは、Topcoat が、接続してきた相手のアドレス（`RemoteAddr`）をリクエストに付けます。テストでは、それを自分で付けます。場面の区切りも標準エラー出力に書くので、サーバーが書いた行と、同じ順に並びます。

```rust
//! 第8巻 UNIT 10: 「攻撃されたかもしれない」と思ったときに、あとから読める記録が残るかを試す。
//! 攻撃する人（203.0.113.7）と alice（198.51.100.20）のリクエストを順に送り、
//! サーバーが標準エラー出力に書いた行を、そのまま見る。
//!
//! 実行: cargo test --test u10_audit -- --nocapture

mod common;

use common::{encode, session_cookie, test_app};
use topcoat::router::{
    Body, Method, RemoteAddr, Router, header, request::Request, response::Response,
};

const PASSWORD: &str = "correct horse battery";
const ATTACKER: &str = "203.0.113.7:51000";
const ALICE: &str = "198.51.100.20:62000";

/// どこから来たか（接続の相手のアドレス）を付けて送る
async fn send(
    router: &Router,
    from: &str,
    path: &str,
    form: &str,
    cookie: Option<&str>,
    cross_site: bool,
) -> Response {
    let mut builder = Request::builder()
        .method(Method::POST)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .extension(RemoteAddr(from.parse().unwrap()));
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    if cross_site {
        builder = builder.header("sec-fetch-site", "cross-site");
    }
    router
        .handle(builder.body(Body::from(form.to_string())).unwrap())
        .await
}

fn login_form(email: &str, password: &str) -> String {
    format!("email={}&password={}", encode(email), encode(password))
}

#[tokio::test]
async fn what_is_left_in_the_log() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    let router = &app.router;
    eprintln!("=== ここから ===");

    eprintln!("--- 1. 攻撃する人が、alice のパスワードを 2 回まちがえる");
    for guess in ["password", "123456"] {
        send(
            router,
            ATTACKER,
            "/login",
            &login_form("alice@example.com", guess),
            None,
            false,
        )
        .await;
    }

    eprintln!("--- 2. alice が、メールの欄にパスワードを打ってしまう");
    send(
        router,
        ALICE,
        "/login",
        &login_form(PASSWORD, ""),
        None,
        false,
    )
    .await;

    eprintln!("--- 3. alice がログインする");
    let response = send(
        router,
        ALICE,
        "/login",
        &login_form("alice@example.com", PASSWORD),
        None,
        false,
    )
    .await;
    let alice = session_cookie(&response).unwrap();

    eprintln!("--- 4. ほかのサイトのページが、alice のブラウザに POST /notes を送らせる");
    let response = send(router, ALICE, "/notes", "title=x&body=", Some(&alice), true).await;
    eprintln!("    （返事は {}）", response.status());

    eprintln!("--- 5. alice がログアウトする");
    send(router, ALICE, "/logout", "", Some(&alice), false).await;
    eprintln!("=== ここまで ===");
}
```

```powershell
cargo test --test u10_audit -- --nocapture
```

```text
=== ここから ===
--- 1. 攻撃する人が、alice のパスワードを 2 回まちがえる
POST /login -> 401 Unauthorized (388.3152ms)
POST /login -> 401 Unauthorized (400.7529ms)
--- 2. alice が、メールの欄にパスワードを打ってしまう
POST /login -> 401 Unauthorized (817.2831ms)
--- 3. alice がログインする
POST /login -> 303 See Other (438.5186ms)
--- 4. ほかのサイトのページが、alice のブラウザに POST /notes を送らせる
    （返事は 403 Forbidden）
--- 5. alice がログアウトする
POST /logout -> 303 See Other (32.9143ms)
=== ここまで ===
```

- **分かること:** 何のリクエストが、どう返ったか（`POST /login -> 401`）と、かかった時間です。
- **分からないこと:** ①と②の `401` が、**誰の**アカウントへの失敗なのか、**どこから**来たのかが、ありません。①は攻撃、②は alice の打ちまちがいですが、記録からは見分けられません。
- **残らないこと:** ④の `403` は、どこにもありません。攻撃は止まっていますが、「止めた」という記録もありません。
- ②だけ約 0.8 秒なのは、例10-1の解説2と同じく、`DUMMY_HASH` の最初の1回だからです（メールの形ではないので、登録の無いメールと同じく、ダミーで照合します）。

#### 例10-3: ログインとログアウトの、監査の記録を足す

> **▼ このコードがやること（先に日本語で）:** RustNote のコピーに、`src/audit.rs` を足します。`record` は、「いつ（1970 年からの秒数）・どこから（`client_ip`）・誰が・何をして・どうなったか」を、`[audit]` で始まる1行にして、標準エラー出力に書きます。ログインに失敗したときのメールアドレスは、先頭の1文字と、`@` から後ろだけにします。`@` が無いときは、何も残しません。

```rust
//! 監査の記録（第8巻 UNIT 10 で、RustNote のコピーに足して試す）。
//!
//! 「いつ・どこから・誰が・何をして・どうなったか」を、`[audit]` で始まる1行にして
//! 標準エラー出力に書く。パスワード・トークン・クッキーの値は、書かない。

use std::time::{SystemTime, UNIX_EPOCH};

use topcoat::{context::Cx, router::request::client_ip};

use crate::domain::UserId;

/// 記録する出来事
pub enum Event<'a> {
    /// ログインできた
    LoginSucceeded(UserId),
    /// ログインに失敗した（入力されたメールアドレス）
    LoginFailed(&'a str),
    /// ログアウトした（ログインしていなかったときは None）
    LoggedOut(Option<UserId>),
}

/// メールアドレスを、先頭の1文字と @ から後ろだけにする（alice@example.com → a…@example.com）。
/// @ が無いときは、何も残さない（メールの欄に、パスワードを打ってしまった人がいるため）
fn mask_email(email: &str) -> String {
    match email.split_once('@') {
        Some((name, domain)) => {
            let first: String = name.chars().take(1).collect();
            format!("{first}…@{domain}")
        }
        None => "（メールアドレスの形ではない）".to_string(),
    }
}

/// 1行の記録を書く
pub fn record(cx: &Cx, event: Event<'_>) {
    // 1970 年 1 月 1 日からの秒数
    let at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0);
    let ip = client_ip(cx).map_or("-".to_string(), |ip| ip.to_string());
    let what = match event {
        Event::LoginSucceeded(user) => format!("login_succeeded user={user}"),
        Event::LoginFailed(email) => format!("login_failed email={}", mask_email(email)),
        Event::LoggedOut(Some(user)) => format!("logged_out user={user}"),
        Event::LoggedOut(None) => "logged_out user=-".to_string(),
    };
    eprintln!("[audit] at={at} ip={ip} {what}");
}
```

`lib.rs` に `pub mod audit;` を足し、ログインとログアウトのページで、結果が決まった所から呼びます。

```diff
--- a/src/routes/login.rs
+++ b/src/routes/login.rs
@@ -12,6 +12,7 @@
     view::{View, component, view},
 };
 
+use crate::audit::{self, Event};
 use crate::auth::{authenticate, current_user, log_in};
 use crate::flash::{self, Flash};
 use crate::views::flash_banner;
@@ -77,9 +78,11 @@
 pub async fn login(cx: &Cx, Form(input): Form<LoginForm>) -> Result<impl View> {
     if let Some(user) = authenticate(cx, &input.email, &input.password).await? {
         log_in(cx, &user).await?;
+        audit::record(cx, Event::LoginSucceeded(user.id));
         flash::set(cx, Flash::LoggedIn);
         return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
     }
+    audit::record(cx, Event::LoginFailed(&input.email));
     // メールが無いのか、パスワードが違うのかは区別しない
     let error = Some(LOGIN_FAILED.to_string());
     Ok(view! {
```

```diff
--- a/src/routes/logout.rs
+++ b/src/routes/logout.rs
@@ -9,14 +9,17 @@
     router::{error::see_other, href, page},
 };
 
-use crate::auth::log_out;
+use crate::audit::{self, Event};
+use crate::auth::{current_user, log_out};
 use crate::flash::{self, Flash};
 
 // レイアウト（エラー画面）を通すため #[page(POST)] にしている。成功すると必ず 303 なので、
 // 画面として返す中身は無い（`()`）。
 #[page(POST)]
 pub async fn logout(cx: &Cx) -> Result<()> {
+    let user = current_user(cx).await?.map(|user| user.id);
     log_out(cx).await?;
+    audit::record(cx, Event::LoggedOut(user));
     flash::set(cx, Flash::LoggedOut);
     Err(see_other(href!(crate::routes::login::login_page).resolve(cx)).into())
 }
```

例10-2と同じテストを、もう一度動かします。

```powershell
cargo test --test u10_audit -- --nocapture
```

```text
=== ここから ===
--- 1. 攻撃する人が、alice のパスワードを 2 回まちがえる
[audit] at=1793377607 ip=203.0.113.7 login_failed email=a…@example.com
POST /login -> 401 Unauthorized (455.926ms)
[audit] at=1793377608 ip=203.0.113.7 login_failed email=a…@example.com
POST /login -> 401 Unauthorized (464.4212ms)
--- 2. alice が、メールの欄にパスワードを打ってしまう
[audit] at=1793377609 ip=198.51.100.20 login_failed email=（メールアドレスの形ではない）
POST /login -> 401 Unauthorized (798.9436ms)
--- 3. alice がログインする
[audit] at=1793377609 ip=198.51.100.20 login_succeeded user=1
POST /login -> 303 See Other (409.3558ms)
--- 4. ほかのサイトのページが、alice のブラウザに POST /notes を送らせる
    （返事は 403 Forbidden）
--- 5. alice がログアウトする
[audit] at=1793377609 ip=198.51.100.20 logged_out user=1
POST /logout -> 303 See Other (5.5758ms)
=== ここまで ===
```

- ①は「`203.0.113.7` から、`a…@example.com` へのログインの失敗が2回」と読めます。②は「alice と同じ `198.51.100.20` から、メールの形ではない入力」です。①と②を、記録だけで見分けられるようになりました。
- ③の `login_succeeded user=1` と、⑤の `logged_out user=1` で、alice（`users.id` が 1）が、いつからいつまでログインしていたかが分かります。
- ④の `403` は、監査の記録を足しても残りません（図の2つ目）。Origin の検査は、ルーターが一番外側に置く層で、断ると、内側の層もページも動かないからです。topcoat-router 0.11.0 の中には、ログを書く所がありません。数えたいときは、`Router::handle` を呼ぶ側（テストなら、例10-2の `（返事は 403 Forbidden）` の行）で、状態コードを見る必要があります。RustNote では、攻撃が止まっていることを、例10-1の守りのテストで毎回確かめることにして、403 は記録しません。

> **注意:** 本物のサーバーを、プロキシ（入口の中継のサーバー）の後ろで動かすと、`client_ip` は、プロキシのアドレスになります。利用者のアドレスを記録したいときは、Topcoat の `TrustedProxies` で、どのプロキシが付けるヘッダーを信じるかを設定します。

#### 例10-4: 事故の練習（DB が漏れたかもしれない）

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録してから、2つのことを確かめます。1つ目は、漏れた DB から読める値（セッションの `token_hash` と、パスワードの `password_hash`）を、攻撃する人がそのまま使ったらどうなるかです。`token_hash` は、Topcoat の `Token::new(…).encode()` で、本物のクッキーと同じ形（32 バイトを base64 にした文字列）にしてから送ります。2つ目は、`DELETE FROM sessions` で全員のセッションを一度に消したら、2人のクッキーがどうなるかです。

```rust
//! 第8巻 UNIT 10: 事故の練習。「DB の中身が漏れたかもしれない」とわかったときに、
//! 漏れた DB の値で、攻撃する人が何をできるかと、全員のセッションを、
//! サーバーの側で一度に終わらせられるかを確かめる。
//!
//! 実行: cargo test --test u10_incident -- --nocapture

mod common;

use common::{TestApp, encode, test_app};
use topcoat::session::Token;

const PASSWORD: &str = "correct horse battery";

/// 先頭の4バイトを、16 進数で表示する
fn head(bytes: &[u8]) -> String {
    bytes[..4]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// sessions 表の、最初の行の token_hash（32 バイト）
async fn leaked_token_hash(app: &TestApp) -> [u8; 32] {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT token_hash FROM sessions LIMIT 1", ())
        .await
        .unwrap();
    let bytes: Vec<u8> = rows.next().await.unwrap().unwrap().get(0).unwrap();
    bytes.try_into().unwrap()
}

/// users 表の、最初の行の password_hash（PHC 文字列）
async fn leaked_password_hash(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT password_hash FROM users LIMIT 1", ())
        .await
        .unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

#[tokio::test]
async fn leaked_database() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;

    // 1. 漏れた DB の値を、そのまま使ってみる
    let hash = leaked_token_hash(&app).await;
    let password_hash = leaked_password_hash(&app).await;
    println!("1. 漏れた token_hash: {}…（32 バイト）", head(&hash));
    println!("   漏れた password_hash: {}…", &password_hash[..30]);
    // token_hash を、本物のクッキーと同じ形（32 バイトの base64）にして送る
    let forged = Token::new(hash);
    let cookie = format!("__Host-session={}", forged.encode());
    let status = app.get("/notes", Some(&cookie)).await.status();
    println!("   token_hash をクッキーの形にして GET /notes → {status}");
    println!(
        "   （サーバーが DB で探すのは、その SHA-256: {}…）",
        head(forged.hash().as_slice())
    );
    // PHC 文字列を、そのままパスワードとして送る
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode(&password_hash)
    );
    let status = app.post_form("/login", &form, None).await.status();
    println!("   password_hash をパスワードにしてログイン → {status}");

    // 2. 全員のセッションを終わらせる
    let conn = app.db.db.connect().await.unwrap();
    let deleted = conn.execute("DELETE FROM sessions", ()).await.unwrap();
    println!("2. DELETE FROM sessions → {deleted} 行");
    for (who, cookie) in [("alice", &alice), ("bob", &bob)] {
        let status = app.get("/notes", Some(cookie)).await.status();
        println!("   {who} のクッキーで GET /notes → {status}");
    }
}
```

```powershell
cargo test --test u10_incident -- --nocapture
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.65s
     Running tests\u10_incident.rs (target\debug\deps\u10_incident-ba939ce4ce45f336.exe)

running 1 test
POST /signup -> 303 See Other (519.3646ms)
POST /signup -> 303 See Other (457.4469ms)
1. 漏れた token_hash: 7a06ad2a…（32 バイト）
   漏れた password_hash: $argon2id$v=19$m=19456,t=2,p=1…
GET /notes -> 303 See Other (5.0705ms)
   token_hash をクッキーの形にして GET /notes → 303 See Other
   （サーバーが DB で探すのは、その SHA-256: 88ef522f…）
POST /login -> 401 Unauthorized (424.7284ms)
   password_hash をパスワードにしてログイン → 401 Unauthorized
2. DELETE FROM sessions → 2 行
GET /notes -> 303 See Other (13.6784ms)
   alice のクッキーで GET /notes → 303 See Other
GET /notes -> 303 See Other (2.4851ms)
   bob のクッキーで GET /notes → 303 See Other
test leaked_database ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.60s
```

- **ハッシュでは入れない:** `token_hash` を、本物のクッキーと同じ形にして送っても、`303`（ログイン画面へ）でした。サーバーは、受け取ったトークンを**もう一度 SHA-256 にかけてから** DB で探します（UNIT 05）。出力の2つの16進数のとおり、探す値は、DB の `token_hash` とは別の値になるので、見つかりません。`password_hash` をパスワードにしても `401` でした。照合は、送られた文字列を Argon2id にかけて比べるので、ハッシュの文字列は、正しいパスワードになりません（UNIT 02）。
- **全員のセッションを終わらせる:** `DELETE FROM sessions` の1文で2行が消え、alice も bob も、次のリクエストから `303` になりました。サーバーの側で DB の記録を消せば、クッキーが利用者のブラウザ（や攻撃する人の手元）に残っていても、もう使えません。代わりに、全員がもう一度ログインします。
- **これだけでは足りないこと:** パスワードのハッシュは、DB の外で、時間をかけて当てられるかもしれません（問2）。

事故に気づいてからの順番を、先に決めておきます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<line x1="70" y1="50" x2="650" y2="50" stroke="#94a3b8" stroke-width="3"/>
<circle cx="70" cy="50" r="22" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="70" y="56" font-size="16" font-weight="700" fill="#b91c1c" text-anchor="middle">1</text>
<circle cx="186" cy="50" r="22" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="186" y="56" font-size="16" font-weight="700" fill="#c2410c" text-anchor="middle">2</text>
<circle cx="302" cy="50" r="22" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="302" y="56" font-size="16" font-weight="700" fill="#1d4ed8" text-anchor="middle">3</text>
<circle cx="418" cy="50" r="22" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="418" y="56" font-size="16" font-weight="700" fill="#6d28d9" text-anchor="middle">4</text>
<circle cx="534" cy="50" r="22" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="534" y="56" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">5</text>
<circle cx="650" cy="50" r="22" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="650" y="56" font-size="16" font-weight="700" fill="#334155" text-anchor="middle">6</text>
<text x="70" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">気づく</text>
<text x="186" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">止める</text>
<text x="302" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">調べる</text>
<text x="418" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">知らせる</text>
<text x="534" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">直す</text>
<text x="650" y="100" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">振り返る</text>
<text x="70" y="124" font-size="10" fill="#475569" text-anchor="middle">記録・問い合わせ</text>
<text x="70" y="142" font-size="10" fill="#475569" text-anchor="middle">「漏れたかも」</text>
<text x="186" y="124" font-size="10" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">DELETE FROM</text>
<text x="186" y="142" font-size="10" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">sessions</text>
<text x="186" y="160" font-size="10" fill="#475569" text-anchor="middle">DB のトークンも</text>
<text x="186" y="178" font-size="10" fill="#475569" text-anchor="middle">（第7巻 UNIT 10）</text>
<text x="302" y="124" font-size="10" fill="#475569" text-anchor="middle">監査の記録で</text>
<text x="302" y="142" font-size="10" fill="#475569" text-anchor="middle">いつ・誰が・どこから</text>
<text x="418" y="124" font-size="10" fill="#475569" text-anchor="middle">利用者に知らせ</text>
<text x="418" y="142" font-size="10" fill="#475569" text-anchor="middle">パスワードの変更を</text>
<text x="418" y="160" font-size="10" fill="#475569" text-anchor="middle">頼む</text>
<text x="534" y="124" font-size="10" fill="#475569" text-anchor="middle">原因を直し</text>
<text x="534" y="142" font-size="10" fill="#475569" text-anchor="middle">守りのテストを足す</text>
<text x="650" y="124" font-size="10" fill="#475569" text-anchor="middle">手順書を直し</text>
<text x="650" y="142" font-size="10" fill="#475569" text-anchor="middle">次に備える</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 事故に気づいてからの順番。先に止めて（被害を広げない）、それから記録で調べる。直したら、守りのテストに1つ足す</div>
</div>

- **先に止める:** 調べている間にも、盗まれたセッションは使えてしまいます。だから、2の「止める」を、3の「調べる」より先にします。記録は消えないので、止めたあとでも調べられます。
- **DB のトークン:** DB に入るためのトークン（`DATABASE_AUTH_TOKEN`）まで漏れたかもしれないときは、第7巻 UNIT 10 の順番で、古いトークンを無効にしてから、新しいトークンに替えます。

### よくあるエラーと読み方

#### エラー1（テストが止まる）: ログアウトで、DB の記録を消さなかった

UNIT 05 のエラー1と同じく、`log_out` で、クッキーだけを消し、DB の記録を消さないようにした場合です。例10-1の守りのテストを動かします。

```text
✓ 認可: ログインしていなければ 303、他人のノートはどの入口でも 404
✓ ログイン: 登録の無いメールも、違うパスワードも、同じ画面
✓ セッション: ログインのたびに新しいトークン、前のトークンは使えない
POST /logout -> 303 See Other (4.0882ms)
GET /notes -> 200 OK (7.0256ms)
thread 'guards' (219200) panicked at tests\u10_guards.rs:80:5:
assertion `left == right` failed: ログアウトのあと、写し取ったクッキーは使えない
  left: 200
 right: 303
test guards ... FAILED
```

- **何が起きたか:** 3つの `✓` のあと、「ログアウトのあと、写し取ったクッキーは使えない」という**守りの名前**で止まりました。ログアウトのあとの `GET /notes` が、`303` ではなく `200 OK`（ノートの一覧が見えた）だったからです。
- **読み方:** `left` が実際の値、`right` が期待した値です。`panicked at tests\u10_guards.rs:80:5` の行番号から、どの `assert_eq!` かも分かります。
- **気をつけること:** 最後の CSRF の守りは、確かめられないまま終わりました（`✓ CSRF` がありません）。1つのテスト関数は、最初の失敗で止まるからです。守りごとに、外れたかどうかを全部知りたいときは、守りごとにテスト関数を分けます（第11巻 UNIT 06（準備中））。

#### エラー2: 記録に、フォームをまるごと書こうとした（E0277）

例10-3の `audit::record(cx, Event::LoginFailed(&input.email));` を、「全部書いておけば、あとで困らない」と、次の1行にした場合です。

```rust
    eprintln!("[audit] login_failed {input:?}");
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0277]: `LoginForm` doesn't implement `Debug`
  --> src\routes\login.rs:85:37
   |
85 |     eprintln!("[audit] login_failed {input:?}");
   |                                     ^^^^^^^^^ `LoginForm` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `LoginForm`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `LoginForm` with `#[derive(Debug)]`
   |
25 + #[derive(Debug)]
26 | pub struct LoginForm {
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustnote` (lib) due to 1 previous error
```

- **何が起きたか:** 「`LoginForm` は `Debug` を持っていないので、`{:?}` で表示できません」というエラーです。`{:?}` は、`Debug` という trait（表示のしかた）を持つ型だけに使えます（第4巻）。
- **読み方:** 最後の `help:` は、「`LoginForm` に `#[derive(Debug)]` を付けてはどうか」と勧めています。コンパイルを通すだけなら、その通りです。でも、その前に、エラー3を読んでください。

#### エラー3（エラーが出ない間違い）: `Debug` を足したら、パスワードが記録に出た

エラー2の `help:` の通りに、`LoginForm` に `#[derive(Debug)]` を付けると、コンパイルは通ります。例10-2と同じテストを動かします。

```text
--- 1. 攻撃する人が、alice のパスワードを 2 回まちがえる
[audit] login_failed LoginForm { email: "alice@example.com", password: "password" }
[audit] login_failed LoginForm { email: "alice@example.com", password: "123456" }
--- 2. alice が、メールの欄にパスワードを打ってしまう
[audit] login_failed LoginForm { email: "correct horse battery", password: "" }
--- 3. alice がログインする
--- 4. ほかのサイトのページが、alice のブラウザに POST /notes を送らせる
--- 5. alice がログアウトする
```

- **何が起きたか:** 記録に、**パスワードがそのまま**出ました。①は、攻撃する人が試したパスワード（`password`・`123456`）です。②は、alice が、メールの欄に打ってしまった**本物のパスワード**（`correct horse battery`）です。
- **困ること:** 記録は、たくさんの人や仕組みが読み、長く残ります。記録を読める人なら誰でも、alice のパスワードを知ることができます。攻撃する人が試したパスワードも、ほかのサイトで、本物の誰かのパスワードかもしれません。
- **直し方:** 記録には、書くと決めた値（例10-3の `Event`）だけを書きます。パスワードを持つ型には、`Debug` を付けないか、自分で書いて中身を隠します。RustNote は、もともと `LoginForm` と `SignupForm` に `Debug` を付けていません。検証済みのパスワードの型 `Password` は、`{:?}` で `Password(***)` と表示されるように書いてあります（`src/domain.rs`）。DB のトークンも `Secret` 型に包んでいます（`src/config.rs`）。

### RustNote ではここで使う

- **守りのテスト:** RustNote の `tests/http.rs` には、この巻の守りのうち、ログインしていないときの 303（`notes_require_login`）・他人のノートの 404（`other_users_notes_are_404`。`POST /notes/{id}/edit` まで確かめている）・ログアウトのあとのクッキー（`login_and_logout`）・ほかのサイトからの POST の 403（`cross_site_post_is_403`）が入っています。
- **まだ入っていない守り（改善点）:** 例10-1の「ログインのたびにトークンが変わり、前のトークンは使えない」と、「失敗の画面が、メールの所を除いて同じ」は、`tests/http.rs` にありません（`login_and_logout` は、失敗の文が入っているかだけを見ています）。第11巻 UNIT 06（準備中） で、守りの回帰テストとして足していきます。
- **監査の記録（改善点）:** 今の RustNote が残すのは、例10-2のアクセスログだけです。例10-3の `audit.rs` は、この単元の練習としてコピーに足したもので、`Rust/rustnote` には入れていません。記録の形（1行の文字列）を、あとで探しやすい形にする方法（`tracing`）は、第11巻 UNIT 07（準備中） で扱います。
- **事故の対応:** 全員のセッションを終わらせるのは `DELETE FROM sessions` の1文、1人だけなら `DELETE FROM sessions WHERE user_id = ?` です（UNIT 05 の例5-2）。どちらも、RustNote には画面がありません。本番の DB に SQL を送って行います。

### 練習問題

**問1:** 例10-3の監査の記録で、攻撃する人が、たくさんのアカウントのパスワードを順に試している（ログインの総当たり）と気づくには、どの値を数えればよいでしょうか。気づいたあと、RustNote に何を足すとよいでしょうか。

**問2:** 「DB が漏れたかもしれない」とき、例10-4の `DELETE FROM sessions` だけでは足りないのは、どんなことでしょうか。

**問3:** 例10-1の守りのテストの、CSRF の確かめから、`.header("sec-fetch-site", "cross-site")` の1行を消すと、どうなるでしょうか。なぜでしょうか。

### 解答と解説

**問1の解答:**

- 例10-3の記録で、同じ `ip=` からの `login_failed` の数を、決まった時間（たとえば1分）ごとに数えます。同じ `email=` への `login_failed` の数も数えます。1つの `ip=` から、いろいろな `email=` への失敗が続けば、総当たりです。ただし、`email=` は伏せてあるので（`a…@example.com`）、正確に数えたいときは、伏せる代わりに、メールアドレスの SHA-256 の先頭の数文字を書く、という方法もあります。
- 気づいたあとは、UNIT 04 の例（`examples/u04_limiter.rs`）のような、回数の制限を足します。Better Auth は、ログインと登録を、1つの IP から 10 秒に 3 回までにしています（UNIT 08）。
- 数えるのを人の目に任せず、「1分に 20 回を超えたら知らせる」のように、自動で見張るしくみは、第11巻 UNIT 07（準備中） で扱います。

**問2の解答:**

- **パスワード:** `password_hash` は、例10-4のとおり、そのままでは使えません。でも、攻撃する人は、DB の外で、よくあるパスワードを1つずつ Argon2id にかけて、同じハッシュになるものを探せます。Argon2id は1回の計算に時間とメモリがかかるので（UNIT 02）、強いパスワードはまず当たりませんが、弱いパスワードから順に当てられていきます。だから、利用者に事故を知らせ、パスワードを変えてもらいます（同じパスワードを、ほかのサイトでも使っている人には、そちらも）。
- **ノートの中身:** DB には、ノートの題と本文も入っています。セッションを消しても、漏れた中身は戻りません。何が漏れたかを、利用者に正直に知らせます。
- **DB のトークン:** 漏れた原因が、DB に入るトークンだったなら、トークンを無効にしないかぎり、攻撃する人は、また DB を読めます（第7巻 UNIT 10）。セッションを消すより先か、同じ時に止めます。
- **再設定のトークン:** パスワードの再設定（UNIT 09）を足していたら、その表の、まだ使っていないトークンも消します。

**問3の解答:**

試すと、次のように止まります。

```text
POST /notes -> 303 See Other (5.8299ms)
✓ 認可: ログインしていなければ 303、他人のノートはどの入口でも 404
✓ ログイン: 登録の無いメールも、違うパスワードも、同じ画面
✓ セッション: ログインのたびに新しいトークン、前のトークンは使えない
✓ ログアウト: 写し取ったクッキーも使えない
POST /notes -> 303 See Other (9.9293ms)
thread 'guards' (211736) panicked at tests\u10_guards.rs:95:5:
assertion `left == right` failed: ほかのサイトからの POST は 403
  left: 303
 right: 403
```

- 1行目の `POST /notes -> 303` は、最初に alice のノートを作ったときの行です。最後の `POST /notes -> 303` が、CSRF の確かめのリクエストで、403 にならず、**ノートが作られてしまいました**（`303` は、作ったあとの移動）。守りのテストは、「ほかのサイトからの POST は 403」の名前で止まりました。
- RustNote が壊れたのではありません。Topcoat の Origin の検査は、`Sec-Fetch-Site` も `Origin` も無いリクエストを、ブラウザ以外（curl やほかのサーバー）からのものとして通します（UNIT 06 の表）。ブラウザは、ほかのサイトのページからの POST に、必ず `Sec-Fetch-Site: cross-site` を付けます。ヘッダーを消したテストは、「ほかのサイトからの、**ブラウザの**リクエスト」を作れていなかったのです。
- 守りのテストのリクエストは、本物の攻撃と同じ形で作ります。テストが通ったか、止まったかだけでなく、例10-1のように、**アクセスログで、思った通りのリクエストが届いたか**も確かめます。

### まとめ

- この巻の守り（認可・ログインの失敗・セッション・ログアウト・CSRF）を1本のテストにまとめ、守りごとに名前を付けた。DB の記録を消さないログアウトにすると、「ログアウトのあと、写し取ったクッキーは使えない」の名前で止まった。リクエストは本物の攻撃と同じ形で作る（`Sec-Fetch-Site` を消すと、CSRF の確かめにならない）。
- 今の RustNote のアクセスログには、誰が・どこからが無く、Origin の検査の 403 は、どの記録にも残らない。監査の記録は、結果が分かるページの中で、いつ・どこから・誰が・何をして・どうなったかを1行で書く。メールアドレスは伏せ、パスワード・トークン・クッキーの値は書かない。フォームを `{:?}` でまるごと書くと、パスワードが記録に出る。
- 漏れた `token_hash` と `password_hash` では、そのままでは入れなかった。`DELETE FROM sessions` で、全員のセッションを一度に終わらせられる。事故のときは、気づく・止める・調べる・知らせる・直す・振り返るの順に動き、分かった守りを、守りのテストに1つ足す。
