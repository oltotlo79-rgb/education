use libsql::{Builder, Connection};

// 直した版: 検索の言葉は ?2 に値として渡す。% は Rust 側で付ける
async fn search_titles(
    conn: &Connection,
    owner: i64,
    word: &str,
) -> Result<Vec<String>, libsql::Error> {
    let pattern = format!("%{word}%");
    let mut rows = conn
        .query(
            "SELECT title FROM notes WHERE user_id = ?1 AND title LIKE ?2 ORDER BY id",
            libsql::params![owner, pattern],
        )
        .await?;
    let mut titles = Vec::new();
    while let Some(row) = rows.next().await? {
        titles.push(row.get::<String>(0)?);
    }
    Ok(titles)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    for word in ["買い物", "' OR 1=1 --"] {
        println!("{word:?} → {:?}", search_titles(&conn, 1, word).await?);
    }
    Ok(())
}
