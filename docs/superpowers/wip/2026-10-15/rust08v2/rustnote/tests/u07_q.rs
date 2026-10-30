//! 第8巻 UNIT 07 練習問題1: 一覧の検索（GET /notes?q=…）に、bob とログインしていない人が来たとき。
//! 実行: cargo test --test u07_q -- --nocapture

mod common;

use common::{body_text, encode, test_app};

#[tokio::test]
async fn q1_list_for_bob_and_guest() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    let url = format!("/notes?q={}", encode("牛乳"));
    for (who, cookie) in [("bob", Some(bob.as_str())), ("guest", None)] {
        let response = app.get(&url, cookie).await;
        let status = response.status();
        let html = body_text(response).await;
        println!(
            "問1 {who} GET /notes?q=牛乳 → {status}（「牛乳を買う」が出る: {}）",
            html.contains("牛乳を買う</a>")
        );
    }
}
