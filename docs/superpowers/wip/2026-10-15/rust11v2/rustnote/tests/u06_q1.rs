//! 第11巻 UNIT 06 問1: 状態を変える入口を、ほかのサイトのリンクから GET で開かせたら、どうなるか。
//! 実行: cargo test --test u06_q1 -- --nocapture

mod common;

use common::{body_text, test_app};
use topcoat::router::{Body, Method, header, request::Request};

#[tokio::test]
async fn state_changing_urls_by_get() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "alice の秘密", "").await;

    for path in ["/notes/1/toggle", "/notes/1/delete", "/logout"] {
        // ほかのサイトのリンク（<a href>）を押すと、GET で開かれる。SameSite=Lax でも、移動の GET にはクッキーが付く
        let request = Request::builder()
            .method(Method::GET)
            .uri(path)
            .header(header::COOKIE, alice.as_str())
            .header("sec-fetch-site", "cross-site")
            .body(Body::empty())
            .unwrap();
        let response = app.router.handle(request).await;
        let allow = response
            .headers()
            .get(header::ALLOW)
            .map(|v| v.to_str().unwrap().to_string());
        println!(
            "ほかのサイトから GET {path} → {}（Allow: {allow:?}）",
            response.status()
        );
    }
    let html = body_text(app.get("/notes/1", Some(&alice)).await).await;
    println!(
        "alice のノートは残っている: {}・完了になっていない: {}",
        html.contains("alice の秘密"),
        !html.contains(r#"<span class="badge">完了</span>"#)
    );
    let status = app.get("/notes", Some(&alice)).await.status();
    println!("alice はログインしたまま: {status}");
}
