use std::time::{Duration, Instant};

use async_lab::job_queue;
use libsql::Connection;
use tokio_util::sync::CancellationToken;

// 止める合図が来たら、今の仕事を終えてから止まる worker
async fn worker(
    conn: Connection,
    token: CancellationToken,
    start: Instant,
) -> Result<u32, libsql::Error> {
    let mut done = 0;
    // 仕事と仕事の間でだけ、合図を確かめる（仕事の途中では止まらない）
    while !token.is_cancelled() {
        match job_queue::claim(&conn, Duration::from_secs(30)).await? {
            Some(job) => {
                // 1件に 200ms かかる
                tokio::time::sleep(Duration::from_millis(200)).await;
                job_queue::complete(&conn, job.id).await?;
                done += 1;
                println!(
                    "{:.1} 秒: #{} を処理しました",
                    start.elapsed().as_secs_f64(),
                    job.id
                );
            }
            // 仕事が無いときは、少し待つ。待っている間に合図が来たら、すぐに止まる
            None => tokio::select! {
                _ = token.cancelled() => break,
                _ = tokio::time::sleep(Duration::from_millis(50)) => {}
            },
        }
    }
    Ok(done)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10q2");
    std::fs::create_dir_all("u10q2").expect("u10q2 フォルダを作れません");
    let db = job_queue::open("u10q2/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    for n in 1..=10 {
        job_queue::enqueue(&conn, "make_report", &format!("報告書{n}"), 3).await?;
    }

    let start = Instant::now();
    let token = CancellationToken::new();
    let handle = tokio::spawn(worker(job_queue::connect(&db).await?, token.clone(), start));

    // 0.5 秒後に止める合図を出す（Ctrl+C を受け取ったつもり）
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!(
        "{:.1} 秒: 止める合図を出します",
        start.elapsed().as_secs_f64()
    );
    token.cancel();

    let done = handle.await.expect("worker が panic しました")?;
    println!(
        "{:.1} 秒: worker が止まりました（処理した仕事 {done} 件）",
        start.elapsed().as_secs_f64()
    );
    println!("残りの件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
