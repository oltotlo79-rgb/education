//! 第10巻 UNIT 08: ノートが 20 万件あるときに、最後のページを OFFSET で読むのと、
//! cursor（続きの印）で読むのとで、DB の読み方と時間を比べる。
//! 実行: cargo test --test u08_deep -- --nocapture

mod common;

use std::time::Instant;

use common::test_app;

const ROWS: i64 = 200_000;

#[tokio::test]
async fn deep_page() {
    let app = test_app().await;
    app.sign_up("alice@example.com", "correct horse battery")
        .await;
    let conn = app.db.db.connect().await.unwrap();

    // 20 万件を SQL で一度に作る（1秒ずつ違う updated_at）
    let started = Instant::now();
    conn.execute(
        "INSERT INTO notes (user_id, title, updated_at)
         WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < ?1)
         SELECT 1, 'ノート' || i, datetime('2026-01-01', '+' || i || ' seconds') FROM n",
        [ROWS],
    )
    .await
    .unwrap();
    println!("{ROWS} 件を作った（{:.2?}）", started.elapsed());

    let offset_sql = "SELECT title FROM notes WHERE user_id = 1
                      ORDER BY updated_at DESC, id DESC LIMIT 20 OFFSET ?1";
    // 最後から2番目のページの、最後の行を続きの印にする（ノート21）
    let cursor_sql = "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
                      ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn
        .query(
            "SELECT updated_at, id FROM notes WHERE title = 'ノート21'",
            (),
        )
        .await
        .unwrap();
    let row = rows.next().await.unwrap().unwrap();
    let cursor: (String, i64) = (row.get(0).unwrap(), row.get(1).unwrap());

    for (name, sql) in [("OFFSET", offset_sql), ("cursor", cursor_sql)] {
        let mut plan = conn
            .query(&format!("EXPLAIN QUERY PLAN {sql}"), ())
            .await
            .unwrap();
        while let Some(row) = plan.next().await.unwrap() {
            println!("{name} の読み方: {}", row.get::<String>(3).unwrap());
        }
    }
    for round in 1..=3 {
        let started = Instant::now();
        let mut rows = conn.query(offset_sql, [ROWS - 20]).await.unwrap();
        let mut offset_titles = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            offset_titles.push(row.get::<String>(0).unwrap());
        }
        let offset_time = started.elapsed();

        let started = Instant::now();
        let mut rows = conn
            .query(cursor_sql, (cursor.0.as_str(), cursor.1))
            .await
            .unwrap();
        let mut cursor_titles = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            cursor_titles.push(row.get::<String>(0).unwrap());
        }
        let cursor_time = started.elapsed();
        println!(
            "{round} 回目: OFFSET {:>9.2?}（{} 〜 {}）／ cursor {:>9.2?}（{} 〜 {}）",
            offset_time,
            offset_titles[0],
            offset_titles[19],
            cursor_time,
            cursor_titles[0],
            cursor_titles[19]
        );
    }
}
