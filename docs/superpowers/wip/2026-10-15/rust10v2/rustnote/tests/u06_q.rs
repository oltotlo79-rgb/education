//! 第10巻 UNIT 06 練習問題1: ユーザーを消すと、そのユーザーのノートとセッションは、どうなるか。
//! 実行: cargo test --test u06_q -- --nocapture

mod common;

use common::{TestApp, test_app};

/// 表ごとの行の数を、"notes=3 sessions=2" の形にする
async fn counts(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut parts = Vec::new();
    for sql in [
        "SELECT COUNT(*) FROM users",
        "SELECT COUNT(*) FROM notes",
        "SELECT COUNT(*) FROM notes WHERE user_id = 2",
        "SELECT COUNT(*) FROM sessions",
    ] {
        let mut rows = conn.query(sql, ()).await.unwrap();
        let n: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
        parts.push(n.to_string());
    }
    format!(
        "users={} notes={}（うち bob の分 {}） sessions={}",
        parts[0], parts[1], parts[2], parts[3]
    )
}

#[tokio::test]
async fn q1_delete_a_user() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    for title in ["1件目", "2件目", "3件目"] {
        app.create_note(&alice, title, "").await;
    }
    app.create_note(&bob, "bob の1件目", "").await;
    println!("問1 消す前:     {}", counts(&app).await);
    let conn = app.db.db.connect().await.unwrap();
    let deleted = conn
        .execute("DELETE FROM users WHERE email = 'alice@example.com'", ())
        .await
        .unwrap();
    println!("問1 DELETE FROM users … → {deleted} 行");
    println!("問1 alice を消したあと: {}", counts(&app).await);
    let status = app.get("/notes", Some(&alice)).await.status();
    println!("問1 alice のクッキーで GET /notes → {status}");
}
