//! 第11巻 UNIT 07 エラー1: subscriber を入れずに、リクエストを送った。
//! 実行: cargo test --test e07_no_subscriber -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn logs_without_subscriber() {
    let app = test_app().await;
    app.get("/login", None).await;
    app.get("/notes/99", None).await;
    println!("2つのリクエストを送りました");
}
