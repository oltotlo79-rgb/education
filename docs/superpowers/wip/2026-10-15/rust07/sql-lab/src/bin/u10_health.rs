use std::time::{Duration, Instant};

use libsql::{Builder, Connection};

// これより時間がかかった SQL を「遅い」として記録する
const SLOW: Duration = Duration::from_millis(100);

// 浅い確認: SQL を受け付けるかだけを見る
async fn ping(conn: &Connection) -> Result<(), libsql::Error> {
    let mut rows = conn.query("SELECT 1", ()).await?;
    rows.next().await?;
    Ok(())
}

// 深い確認: アプリが使う表が読めて、migration が入っているかまで見る
async fn check(conn: &Connection) -> Result<i64, String> {
    let mut rows = conn
        .query("SELECT COUNT(*) FROM schema_migrations", ())
        .await
        .map_err(|e| e.to_string())?;
    let row = rows
        .next()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("COUNT(*) が行を返しません")?;
    let applied = row.get::<i64>(0).map_err(|e| e.to_string())?;
    if applied == 0 {
        return Err("migration が1つも入っていません".to_string());
    }
    Ok(applied)
}

// SQL を1件送り、かかった時間を測る。遅ければ [warn] を出す
async fn timed_count(conn: &Connection, label: &str, sql: &str) -> Result<i64, libsql::Error> {
    let started = Instant::now();
    let mut rows = conn.query(sql, ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    let value = row.get::<i64>(0)?;
    let elapsed = started.elapsed();
    if elapsed >= SLOW {
        eprintln!("[warn] 遅い SQL（{} ms）: {label}", elapsed.as_millis());
    }
    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10h");
    std::fs::create_dir_all("u10h").expect("u10h フォルダを作れません");

    // 正しい DB を用意する（表と、migration の記録）
    {
        let db = Builder::new_local("u10h/app.db").build().await?;
        let conn = db.connect()?;
        conn.execute_batch(include_str!("../../sql/schema.sql"))
            .await?;
        conn.execute_batch(include_str!("../../sql/seed.sql"))
            .await?;
        conn.execute_batch(
            "CREATE TABLE schema_migrations (
                 version TEXT PRIMARY KEY,
                 applied_at TEXT NOT NULL DEFAULT (datetime('now'))
             );
             INSERT INTO schema_migrations (version) VALUES ('0001_users'), ('0002_notes');",
        )
        .await?;
    }

    // ① 正しいパスと、1文字打ち間違えたパスで、2種類の確認をする
    for path in ["u10h/app.db", "u10h/ap.db"] {
        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        let shallow = match ping(&conn).await {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("失敗: {e}"),
        };
        let deep = match check(&conn).await {
            Ok(n) => format!("ok（migration {n} 件）"),
            Err(e) => format!("失敗: {e}"),
        };
        println!("{path}: SELECT 1 → {shallow} / 表の確認 → {deep}");
    }

    // ② 時間のかかる SQL を送り、遅いものに印を付ける
    let db = Builder::new_local("u10h/app.db").build().await?;
    let conn = db.connect()?;
    let n = timed_count(&conn, "notes の件数", "SELECT COUNT(*) FROM notes").await?;
    println!("notes の件数: {n}");
    let n = timed_count(
        &conn,
        "1 から 300 万まで数える",
        "WITH RECURSIVE c(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM c WHERE n < 3000000)
         SELECT COUNT(*) FROM c",
    )
    .await?;
    println!("数えた数: {n}");
    Ok(())
}
