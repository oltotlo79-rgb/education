use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let mut rows = conn.query("SELECT 42", ()).await?;
    if let Some(row) = rows.next().await? {
        let n = row.get(0)?;
        println!("{n}");
    }
    Ok(())
}
