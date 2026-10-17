use std::time::{Duration, Instant};

use libsql::{Builder, Connection};

// 会員 42 番のノートを、新しい順に 20 件読む問い合わせ
const LIST_SQL: &str = "SELECT id, title FROM notes WHERE user_id = ?1
     ORDER BY updated_at DESC, id DESC LIMIT 20";

// 同じ問い合わせを times 回くり返し、1回あたりの平均時間を返す
async fn average_time(conn: &Connection, times: u32) -> Result<Duration, libsql::Error> {
    let start = Instant::now();
    for _ in 0..times {
        let mut rows = conn.query(LIST_SQL, [42]).await?;
        // 行を最後まで読み切る（読まないと、問い合わせが最後まで実行されない）
        while rows.next().await?.is_some() {}
    }
    Ok(start.elapsed() / times)
}

// 1,000 件のノートを追加するのにかかった時間を返す
async fn insert_time(conn: &Connection) -> Result<Duration, libsql::Error> {
    let start = Instant::now();
    let tx = conn.transaction().await?;
    for n in 0..1000 {
        tx.execute(
            "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
            libsql::params![(n % 1000) + 1, format!("追加 {n}")],
        )
        .await?;
    }
    tx.commit().await?;
    Ok(start.elapsed())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // メモリの中だけの DB に、表と 100,000 件のノートを用意する
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/u04_big.sql"))
        .await?;

    let without_index = average_time(&conn, 50).await?;
    let insert_without = insert_time(&conn).await?;

    conn.execute(
        "CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)",
        (),
    )
    .await?;

    let with_index = average_time(&conn, 50).await?;
    let insert_with = insert_time(&conn).await?;

    println!("一覧の読み出し（1回の平均）");
    println!("  索引なし: {without_index:?}");
    println!("  索引あり: {with_index:?}");
    println!(
        "  → 約 {} 倍速い",
        without_index.as_micros() / with_index.as_micros().max(1)
    );
    println!("1,000 件の追加（合計）");
    println!("  索引なし: {insert_without:?}");
    println!("  索引あり: {insert_with:?}");
    Ok(())
}
