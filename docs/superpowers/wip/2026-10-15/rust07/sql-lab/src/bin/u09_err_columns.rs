use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // SELECT の列の順番を変えたのに、読む側の番号を直し忘れた
    let mut rows = conn
        .query(
            "SELECT user_id, id, title FROM notes WHERE user_id = ?1 ORDER BY id DESC",
            [1],
        )
        .await?;
    while let Some(row) = rows.next().await? {
        // 読む側は「0 = id、1 = user_id」のつもりのまま
        let id = row.get::<i64>(0)?;
        let owner = row.get::<i64>(1)?;
        let title = row.get::<String>(2)?;
        println!("#{id} {title}（持ち主 {owner}）");
    }
    Ok(())
}
