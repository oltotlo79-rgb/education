use libsql::{Builder, Connection};

// users と notes の行数を、1回の SQL でまとめて数える
async fn counts(conn: &Connection) -> Result<(i64, i64), libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT (SELECT COUNT(*) FROM users), (SELECT COUNT(*) FROM notes)",
            (),
        )
        .await?;
    let row = rows.next().await?.expect("この SELECT は必ず1行返す");
    Ok((row.get::<i64>(0)?, row.get::<i64>(1)?))
}

// ファイルが壊れていないかを SQLite に調べてもらう（問題がなければ "ok"）
async fn integrity(conn: &Connection) -> Result<String, libsql::Error> {
    let mut rows = conn.query("PRAGMA integrity_check", ()).await?;
    let row = rows
        .next()
        .await?
        .expect("integrity_check は必ず1行以上返す");
    row.get::<String>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 何回動かしても同じ結果になるよう、前回のフォルダを消して作り直す
    let _ = std::fs::remove_dir_all("u10");
    std::fs::create_dir_all("u10").expect("u10 フォルダを作れません");

    // ① 本番の代わりの DB を作り、練習用のデータを入れる
    let db = Builder::new_local("u10/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    let (users, notes) = counts(&conn).await?;
    println!("app.db: users {users} 件、notes {notes} 件");

    // ② VACUUM INTO で、DB を開いたまま別のファイルへ写しを書き出す
    let backup = "u10/backup-2026-10-21.db";
    conn.execute("VACUUM INTO ?1", [backup]).await?;
    let size = std::fs::metadata(backup).expect("写しがありません").len();
    println!("写しを作りました: {backup}（{size} バイト）");

    // ③ 写しを作った後に、ノートを1件書き足す
    conn.execute(
        "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
        libsql::params![1, "写しの後に書いたノート"],
    )
    .await?;
    let (_, notes) = counts(&conn).await?;
    println!("app.db: 書き足した後の notes {notes} 件");

    // ④ 写しを別の DB として開き、中身と壊れていないかを確かめる
    let copy = Builder::new_local(backup).build().await?;
    let copy_conn = copy.connect()?;
    let (users, notes) = counts(&copy_conn).await?;
    println!("写し: users {users} 件、notes {notes} 件");
    println!("写しの integrity_check: {}", integrity(&copy_conn).await?);
    Ok(())
}
