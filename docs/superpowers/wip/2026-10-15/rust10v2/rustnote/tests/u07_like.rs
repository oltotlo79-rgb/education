//! 第10巻 UNIT 07: SQLite の LIKE が、% と _ と ESCAPE をどう扱うかを、SQL を直接動かして確かめる。
//! 実行: cargo test --test u07_like -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn like_patterns() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for title in [
        "セール 100% オフ",
        "100 円の牛乳",
        "file_name の決め方",
        "file-name を決める",
        r"C:\tmp の掃除",
    ] {
        app.create_note(&alice, title, "").await;
    }
    let conn = app.db.db.connect().await.unwrap();
    let plain = "SELECT title FROM notes WHERE title LIKE ?1 ORDER BY id";
    let escaped = r"SELECT title FROM notes WHERE title LIKE ?1 ESCAPE '\' ORDER BY id";
    let cases = [
        ("%100%%", plain),
        (r"%100\%%", plain),
        (r"%100\%%", escaped),
        ("%file_name%", plain),
        (r"%file\_name%", escaped),
        (r"%\%", plain),
        (r"%\\%", escaped),
    ];
    for (pattern, sql) in cases {
        let mut rows = conn.query(sql, [pattern]).await.unwrap();
        let mut found = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            found.push(row.get::<String>(0).unwrap());
        }
        let escape = if sql.contains("ESCAPE") {
            "ESCAPE あり"
        } else {
            "ESCAPE なし"
        };
        println!("{pattern:<14} {escape} → {found:?}");
    }
}
