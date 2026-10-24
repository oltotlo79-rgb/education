use std::sync::Arc;
use std::time::Duration;

use async_lab::job_queue;
use libsql::Connection;

// 悪い例: 「選ぶ（SELECT）」と「自分のものにする（UPDATE）」を、別々の文で行う
async fn claim_badly(conn: &Connection) -> Result<Option<i64>, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT id FROM jobs WHERE status = 'queued' ORDER BY id LIMIT 1",
            (),
        )
        .await?;
    let Some(row) = rows.next().await? else {
        return Ok(None);
    };
    let id = row.get::<i64>(0)?;
    drop(row);
    drop(rows);
    // 選んでから自分のものにするまでの間に、ほかの worker も同じ仕事を選べてしまう
    tokio::time::sleep(Duration::from_millis(50)).await;
    conn.execute("UPDATE jobs SET status = 'running' WHERE id = ?1", [id])
        .await?;
    Ok(Some(id))
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("e10race");
    std::fs::create_dir_all("e10race").expect("e10race フォルダを作れません");
    let db = Arc::new(job_queue::open("e10race/jobs.db").await?);
    let conn = job_queue::connect(&db).await?;
    job_queue::enqueue(&conn, "send_email", "hanako@example.com", 3).await?;

    let mut handles = Vec::new();
    for name in ["worker1", "worker2"] {
        let db = Arc::clone(&db);
        handles.push(tokio::spawn(async move {
            let conn = job_queue::connect(&db).await?;
            let claimed = claim_badly(&conn).await?;
            println!("{name} が取った仕事: {claimed:?}");
            Ok::<(), libsql::Error>(())
        }));
    }
    for handle in handles {
        handle.await.expect("worker が panic しました")?;
    }
    Ok(())
}
