use std::time::Duration;

use async_lab::job_queue::{self, Job};
use libsql::Connection;

// メールを送る仕事。送った記録を sent_emails 表に残し、同じ仕事では2回送らない
async fn send_email_once(conn: &Connection, job: &Job) -> Result<(), libsql::Error> {
    // job_id は PRIMARY KEY なので、同じ仕事の記録は1つしか入らない
    let inserted = conn
        .execute(
            "INSERT OR IGNORE INTO sent_emails (job_id, address) VALUES (?1, ?2)",
            libsql::params![job.id, job.payload.as_str()],
        )
        .await?;
    if inserted == 1 {
        println!(
            "  {} にメールを送りました（{} 回目）",
            job.payload, job.attempts
        );
    } else {
        println!(
            "  #{} はもう送ってあるので、送りません（{} 回目）",
            job.id, job.attempts
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10q3");
    std::fs::create_dir_all("u10q3").expect("u10q3 フォルダを作れません");
    let db = job_queue::open("u10q3/jobs.db").await?;
    let worker_a = job_queue::connect(&db).await?;
    let worker_b = job_queue::connect(&db).await?;
    worker_a
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS sent_emails (
                 job_id INTEGER PRIMARY KEY,
                 address TEXT NOT NULL
             );",
        )
        .await?;
    job_queue::enqueue(&worker_a, "send_email", "hanako@example.com", 3).await?;
    let lease = Duration::from_millis(300);

    // ① A はメールを送ったが、complete を記録する前に落ちた
    println!("① worker A");
    let job = job_queue::claim(&worker_a, lease)
        .await?
        .expect("仕事がある");
    send_email_once(&worker_a, &job).await?;

    // ② 持ち時間が切れたので、B が同じ仕事をやり直す
    tokio::time::sleep(Duration::from_millis(400)).await;
    println!("② worker B");
    let job = job_queue::claim(&worker_b, lease)
        .await?
        .expect("仕事がある");
    send_email_once(&worker_b, &job).await?;
    job_queue::complete(&worker_b, job.id).await?;

    let mut rows = worker_b
        .query("SELECT COUNT(*) FROM sent_emails", ())
        .await?;
    let row = rows.next().await?.expect("COUNT は1行返す");
    println!("送ったメールの数: {}", row.get::<i64>(0)?);
    Ok(())
}
