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
