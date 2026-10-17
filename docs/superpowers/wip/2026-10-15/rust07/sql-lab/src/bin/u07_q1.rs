use libsql::{Builder, Connection};

// 指定した会員の、完了したノートの数を数える
async fn count_done(conn: &Connection, owner: i64) -> Result<i64, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = ?1 AND done = 1",
            [owner],
        )
        .await?;
    // COUNT(*) は、該当が 0 件でも必ず1行を返す
    match rows.next().await? {
        Some(row) => row.get::<i64>(0),
        None => Ok(0),
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    for owner in [1, 2, 3] {
        println!("会員 {owner}: 完了 {} 件", count_done(&conn, owner).await?);
    }
    Ok(())
}
