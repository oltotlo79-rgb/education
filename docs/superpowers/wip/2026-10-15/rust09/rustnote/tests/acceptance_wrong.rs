//! わざと間違えた受入条件（「他人のノートは 403」）のテスト。
//! 実行: cargo test --test acceptance_wrong

mod common;

use common::test_app;
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

// AC-05（間違い）: 他人のノートを開くと 403 Forbidden になる
#[tokio::test]
async fn ac05_other_users_note_is_forbidden() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    let response = app.get(&url, Some(&bob)).await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
