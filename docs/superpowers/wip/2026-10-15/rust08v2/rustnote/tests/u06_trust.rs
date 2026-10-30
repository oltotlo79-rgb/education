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
