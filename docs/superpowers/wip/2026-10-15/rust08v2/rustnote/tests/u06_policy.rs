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
