//! 第11巻 UNIT 07: tracing の構造化ログ。リクエストを送り、JSON の1行ずつの記録を見る。
//! （RustNote のコピーに、tracing と /metrics を足した版で動かす）
//!
//! 実行: cargo test --test u07_logs -- --nocapture

mod common;

use common::{encode, test_app};

#[tokio::test]
async fn structured_logs() {
    // 受け取る人（subscriber）を入れる。テストでは、標準出力に JSON で書く
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stdout)
        .init();

    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;
    let form = format!("email={}&password=wrong", encode("alice@example.com"));
    app.post_form("/login", &form, None).await;
    app.get("/notes/99", Some(&alice)).await;
    app.post_form("/notes", "title=%00&body=", Some(&alice)).await;
}
