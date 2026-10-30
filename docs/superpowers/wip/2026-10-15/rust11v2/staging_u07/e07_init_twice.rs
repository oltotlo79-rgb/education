//! 第11巻 UNIT 07 エラー2: テストごとに subscriber を入れた（同じテストのファイルに2つ）。
//! 実行: cargo test --test e07_init_twice

mod common;

use common::test_app;

fn start_logging() {
    tracing_subscriber::fmt().json().init();
}

#[tokio::test]
async fn first() {
    start_logging();
    test_app().await.get("/login", None).await;
}

#[tokio::test]
async fn second() {
    start_logging();
    test_app().await.get("/signup", None).await;
}
