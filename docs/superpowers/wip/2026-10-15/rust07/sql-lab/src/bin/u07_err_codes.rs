use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // わざと失敗する文を並べる
    let bad_statements = [
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
        "INSERT INTO users (id, email, password_hash) VALUES (1, 'new@example.com', 'x')",
        "INSERT INTO notes (user_id, title) VALUES (1, NULL)",
        "INSERT INTO notes (user_id, title) VALUES (1, '')",
        "INSERT INTO notes (user_id, title) VALUES (99, '持ち主がいない')",
        "SELEC id FROM notes",
    ];
    for sql in bad_statements {
        let error = conn.execute(sql, ()).await.unwrap_err();
        // {:?} は種類と番号、{} は人が読むための文
        println!("{error:?}");
        println!("  → {error}");
        // 種類と番号で見分ける
        let kind = match &error {
            libsql::Error::SqliteFailure(2067, _) => "UNIQUE 違反（409 にする候補）",
            libsql::Error::SqliteFailure(1555, _) => "主キーの重複",
            libsql::Error::SqliteFailure(1299, _) => "NOT NULL 違反",
            libsql::Error::SqliteFailure(275, _) => "CHECK 違反",
            libsql::Error::SqliteFailure(787, _) => "外部キー違反",
            libsql::Error::SqliteFailure(1, _) => "SQL の書き間違いなど",
            _ => "その他",
        };
        println!("  → 分類: {kind}");
    }
    Ok(())
}
