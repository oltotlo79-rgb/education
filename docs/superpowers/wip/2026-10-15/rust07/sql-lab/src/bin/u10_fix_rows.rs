use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10f");
    std::fs::create_dir_all("u10f").expect("u10f フォルダを作れません");
    let db = Builder::new_local("u10f/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 読むところを {} で囲むと、rows と first はここで捨てられる
    {
        let mut rows = conn.query("SELECT title FROM notes", ()).await?;
        let first = rows.next().await?.expect("ノートは8件ある");
        println!("1件目: {}", first.get::<String>(0)?);
    }
    conn.execute("VACUUM INTO ?1", ["u10f/backup.db"]).await?;
    println!("写しを作りました");
    Ok(())
}
