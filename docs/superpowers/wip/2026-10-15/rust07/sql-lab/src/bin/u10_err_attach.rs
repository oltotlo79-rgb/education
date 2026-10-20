use std::path::Path;

use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10a");
    std::fs::create_dir_all("u10a").expect("u10a フォルダを作れません");
    let db = Builder::new_local("u10a/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    conn.execute("VACUUM INTO ?1", ["u10a/backup.db"]).await?;

    // 写しの名前を1文字打ち間違えた（backup.db のつもり）
    let path = "u10a/backpu.db";
    println!("ATTACH の前に {path} はある?: {}", Path::new(path).exists());
    conn.execute("ATTACH DATABASE ?1 AS bk", [path]).await?;
    println!("ATTACH の後に {path} はある?: {}", Path::new(path).exists());
    let mut rows = conn.query("SELECT COUNT(*) FROM bk.notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    println!("写しの notes: {} 件", row.get::<i64>(0)?);
    Ok(())
}
