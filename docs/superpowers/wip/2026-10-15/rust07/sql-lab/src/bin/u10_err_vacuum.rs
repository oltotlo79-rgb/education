use libsql::Builder;

// execute の結果を「成功」か、エラーの文で表す
fn show(result: Result<u64, libsql::Error>) -> String {
    match result {
        Ok(_) => "成功".to_string(),
        Err(e) => format!("エラー: {e}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10e");
    std::fs::create_dir_all("u10e").expect("u10e フォルダを作れません");
    let db = Builder::new_local("u10e/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // ① 読みかけの rows と row が残ったまま、写しを作ろうとする
    let mut rows = conn.query("SELECT title FROM notes", ()).await?;
    let first = rows.next().await?.expect("ノートは8件ある");
    println!("1件目: {}", first.get::<String>(0)?);
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("① rows と row が残ったまま: {}", show(r));
    drop(rows);
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("① rows だけ捨てた後: {}", show(r));
    drop(first);

    // ② 同じ名前で2回、写しを作ろうとする
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("② 1回目: {}", show(r));
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("② 2回目: {}", show(r));

    // ③ transaction の途中で、写しを作ろうとする
    let tx = conn.transaction().await?;
    let r = tx.execute("VACUUM INTO ?1", ["u10e/backup2.db"]).await;
    println!("③ transaction の中: {}", show(r));
    tx.rollback().await?;
    Ok(())
}
