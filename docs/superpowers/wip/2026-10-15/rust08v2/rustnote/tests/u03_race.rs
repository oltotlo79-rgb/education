//! 第8巻 UNIT 03: 同じメールアドレスの新規登録を、2つ同時に送る（ダブルクリックや、2つのタブ）。
//! 実行: cargo test --test u03_race -- --nocapture

mod common;

use common::{TestApp, encode, test_app};

/// users 表の email を、id の順に
async fn emails(app: &TestApp) -> Vec<String> {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    found
}

#[tokio::test]
async fn two_signups_at_once() {
    let app = test_app().await;
    let password = encode("correct horse battery");
    let form = format!(
        "email={}&password={password}&password_confirm={password}",
        encode("alice@example.com")
    );
    let (a, b) = tokio::join!(
        app.post_form("/signup", &form, None),
        app.post_form("/signup", &form, None),
    );
    println!("1つ目: {}", a.status());
    println!("2つ目: {}", b.status());
    println!("users 表の email: {:?}", emails(&app).await);
}
