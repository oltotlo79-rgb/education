//! 第8巻 UNIT 05: alice のクッキーを写し取った人がいるときに、alice がログアウトすると、
//! 写し取られたクッキーは、まだ使えるかを確かめる。
//! 実行: cargo test --test u05_stolen -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn stolen_cookie_after_logout() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    // 誰かが、alice のクッキーを写し取った（同じ文字列を持っている）
    let stolen = alice.clone();
    let status = app.get("/notes", Some(&stolen)).await.status();
    println!("1. 写し取ったクッキーで GET /notes → {status}");

    let status = app.post_form("/logout", "", Some(&alice)).await.status();
    println!("2. alice が POST /logout → {status}");

    let response = app.get("/notes", Some(&stolen)).await;
    let to = response
        .headers()
        .get("location")
        .map(|v| format!(" → {}", v.to_str().unwrap()))
        .unwrap_or_default();
    println!(
        "3. 写し取ったクッキーで GET /notes → {}{to}",
        response.status()
    );
}
