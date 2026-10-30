//! 問1〜3の解答: ルートの表に足したい3行を、1つずつ確かめる。
//! 実行: cargo test --test u08_q -- --nocapture

mod common;

use common::test_app;
use topcoat::router::{Body, Method, header, request::Request};

#[tokio::test]
async fn more_rows() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;

    // 問1: ページの番号が数ではない
    let request = Request::builder()
        .uri("/notes?page=abc")
        .header(header::COOKIE, &alice)
        .body(Body::empty())
        .unwrap();
    let status = app.router.handle(request).await.status();
    println!("問1 GET  /notes?page=abc（alice）        → {status}");

    // 問2: HEAD で動いているかだけを確かめる
    let request = Request::builder()
        .method(Method::HEAD)
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let status = app.router.handle(request).await.status();
    println!("問2 HEAD /health（未ログイン）            → {status}");

    // 問3: ほかのサイトのページから、ノートを作らせようとする
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, &alice)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(header::ORIGIN, "https://evil.example")
        .header("sec-fetch-site", "cross-site")
        .body(Body::from("title=evil&body="))
        .unwrap();
    let status = app.router.handle(request).await.status();
    println!("問3 POST /notes（alice・ほかのサイトから） → {status}");
}
