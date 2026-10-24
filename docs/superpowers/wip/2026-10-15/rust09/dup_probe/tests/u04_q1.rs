//! 問1の解答の確かめ: POST /notes/{id}/duplicate でノートを複製できる。他人のノートは 404。
//! 実行: cargo test --test u04_q1

mod common;

use common::{body_text, location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn duplicate_copies_own_note_only() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "買い物", "牛乳").await;

    let response = app
        .post_form(&format!("{url}/duplicate"), "", Some(&alice))
        .await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let copy_url = location(&response).to_string();
    println!("{url} を複製 → {copy_url}");
    let html = body_text(app.get(&copy_url, Some(&alice)).await).await;
    assert!(html.contains("買い物") && html.contains("牛乳"));

    let response = app
        .post_form(&format!("{url}/duplicate"), "", Some(&bob))
        .await;
    println!("bob が {url} を複製 → {}", response.status());
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
