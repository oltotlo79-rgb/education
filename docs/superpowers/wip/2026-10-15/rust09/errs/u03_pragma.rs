use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let mut rows = conn.query("PRAGMA foreign_keys", ()).await?;
    let row = rows.next().await?.unwrap();
    println!("foreign_keys = {}", row.get::<i64>(0)?);
    let mut rows = conn.query("PRAGMA busy_timeout", ()).await?;
    let row = rows.next().await?.unwrap();
    println!("busy_timeout = {}", row.get::<i64>(0)?);
    let mut rows = conn.query("SELECT sqlite_version()", ()).await?;
    let row = rows.next().await?.unwrap();
    println!("sqlite_version = {}", row.get::<String>(0)?);
    Ok(())
}
