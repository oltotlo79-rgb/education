use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute("CREATE TABLE memos (id INTEGER PRIMARY KEY, text TEXT)", ())
        .await?;
    let user_text = String::from("牛乳を買う");
    conn.execute("INSERT INTO memos (id, text) VALUES (?1, ?2)", [1, user_text])
        .await?;
    Ok(())
}
