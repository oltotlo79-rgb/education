//! 第8巻 UNIT 05: サーバーの側（sessions 表）だけを変えて、クッキーの持ち主に何が起きるかを確かめる。
//! 実行: cargo test --test u05_server -- --nocapture --test-threads=1

mod common;

use common::{TestApp, encode, session_cookie, test_app};

const PASSWORD: &str = "correct horse battery";

/// sessions 表の、ユーザーごとの行の数（"alice=1 bob=1" の形）
async fn count(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query(
            "SELECT substr(users.email, 1, instr(users.email, '@') - 1), COUNT(sessions.user_id)
               FROM users LEFT JOIN sessions ON sessions.user_id = users.id
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

async fn login(app: &TestApp, email: &str) -> String {
    let form = format!("email={}&password={}", encode(email), encode(PASSWORD));
    let response = app.post_form("/login", &form, None).await;
    session_cookie(&response).unwrap()
}

/// 期限を過去にする
#[tokio::test]
async fn expired_session() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.sign_up("bob@example.com", PASSWORD).await;
    let conn = app.db.db.connect().await.unwrap();
    conn.execute(
        "UPDATE sessions SET expires_at = unixepoch('now') - 1 WHERE user_id = 1",
        (),
    )
    .await
    .unwrap();
    println!(
        "1. alice のセッションの期限を、1秒前にした（sessions: {}）",
        count(&app).await
    );
    let status = app.get("/notes", Some(&alice)).await.status();
    println!(
        "2. alice のクッキーで GET /notes → {status}（sessions: {}）",
        count(&app).await
    );
    login(&app, "bob@example.com").await;
    println!("3. bob がログインした（sessions: {}）", count(&app).await);
}

/// 2つの端末の、両方のセッションを、サーバーの側で消す
#[tokio::test]
async fn revoke_all_devices() {
    let app = test_app().await;
    let pc = app.sign_up("alice@example.com", PASSWORD).await;
    let phone = login(&app, "alice@example.com").await;
    println!(
        "1. PC とスマートフォンでログイン（sessions: {}）",
        count(&app).await
    );
    let conn = app.db.db.connect().await.unwrap();
    let deleted = conn
        .execute("DELETE FROM sessions WHERE user_id = 1", ())
        .await
        .unwrap();
    println!("2. DELETE FROM sessions WHERE user_id = 1 → {deleted} 行");
    for (name, cookie) in [("PC", &pc), ("スマートフォン", &phone)] {
        let status = app.get("/notes", Some(cookie)).await.status();
        println!("3. {name} のクッキーで GET /notes → {status}");
    }
}
