use libsql::{Builder, Connection};

// notes の id を小さい順に全部読む
async fn note_ids(conn: &Connection) -> Result<Vec<i64>, libsql::Error> {
    let mut rows = conn.query("SELECT id FROM notes ORDER BY id", ()).await?;
    let mut ids = Vec::new();
    while let Some(row) = rows.next().await? {
        ids.push(row.get::<i64>(0)?);
    }
    Ok(ids)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10r");
    std::fs::create_dir_all("u10r").expect("u10r フォルダを作れません");

    // ① 本番の代わりの DB を作り、夜のうちに写しを取っておく
    let db = Builder::new_local("u10r/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    conn.execute("VACUUM INTO ?1", ["u10r/backup.db"]).await?;

    // ② 次の朝、Alice が新しいノートを書く（写しには入っていない）
    conn.execute(
        "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
        libsql::params![1, "朝に書いたノート"],
    )
    .await?;
    println!("事故の前: {:?}", note_ids(&conn).await?);

    // ③ 事故: #3 だけ消すつもりが、WHERE を書き忘れた
    let deleted = conn.execute("DELETE FROM notes", ()).await?;
    println!("DELETE で消えた行: {deleted} 件（1件のつもりだった）");
    println!("事故の後: {:?}", note_ids(&conn).await?);

    // ④ 写しを「bk」という名前で横に付け、消えた行だけを写しから戻す
    conn.execute("ATTACH DATABASE ?1 AS bk", ["u10r/backup.db"])
        .await?;
    let tx = conn.transaction().await?;
    let restored = tx
        .execute(
            "INSERT INTO main.notes
                 (id, user_id, title, body, done, version, created_at, updated_at)
             SELECT id, user_id, title, body, done, version, created_at, updated_at
             FROM bk.notes
             WHERE id NOT IN (SELECT id FROM main.notes)",
            (),
        )
        .await?;
    tx.commit().await?;
    conn.execute("DETACH DATABASE bk", ()).await?;
    println!("写しから戻した行: {restored} 件");
    println!("戻した後: {:?}", note_ids(&conn).await?);
    Ok(())
}
