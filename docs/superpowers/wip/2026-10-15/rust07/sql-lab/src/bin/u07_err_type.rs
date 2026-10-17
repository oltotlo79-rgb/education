use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 列の順番を取り違えて、title（文字列）を i64 で取り出してしまう
    let mut rows = conn
        .query("SELECT title, id FROM notes WHERE id = 1", ())
        .await?;
    let row = rows.next().await?.expect("ノート 1 はある");
    let id = row.get::<i64>(0)?;
    println!("id = {id}");
    Ok(())
}
