//! 第8巻 UNIT 04 練習問題1: alice でログインしたまま、ログインの画面を開いたり、bob でログインしたりする。
//! 実行: cargo test --test u04_q -- --nocapture

mod common;

use common::{TestApp, body_text, encode, session_cookie, test_app};

const PASSWORD: &str = "correct horse battery";

/// sessions 表の、user_id ごとの行の数
async fn sessions(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query(
            "SELECT users.email, COUNT(sessions.user_id) FROM users
             LEFT JOIN sessions ON sessions.user_id = users.id
             GROUP BY users.id ORDER BY users.id",
            (),
        )
        .await
        .unwrap();
    let mut parts = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        parts.push(format!(
            "{}={}",
            row.get::<String>(0).unwrap(),
            row.get::<i64>(1).unwrap()
        ));
    }
    parts.join(" ")
}

#[tokio::test]
async fn q1_login_while_logged_in() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.sign_up("bob@example.com", PASSWORD).await;
    println!("問1 はじめのセッション: {}", sessions(&app).await);

    let response = app.get("/login", Some(&alice)).await;
    let to = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!(
        "問1 alice のクッキーで GET /login → {} → {to}",
        response.status()
    );

    let form = format!(
        "email={}&password={}",
        encode("bob@example.com"),
        encode(PASSWORD)
    );
    let response = app.post_form("/login", &form, Some(&alice)).await;
    let bob = session_cookie(&response).unwrap();
    println!(
        "問1 alice のクッキーのまま bob でログイン → {}",
        response.status()
    );
    println!("問1 そのあとのセッション: {}", sessions(&app).await);

    let html = body_text(app.get("/notes", Some(&bob)).await).await;
    let who = html
        .split("<span class=\"user-email\">")
        .nth(1)
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("");
    println!("問1 新しいクッキーで一覧を開くと、右上は: {who}");
    let status = app.get("/notes", Some(&alice)).await.status();
    println!("問1 alice の古いクッキーで GET /notes → {status}");
}
