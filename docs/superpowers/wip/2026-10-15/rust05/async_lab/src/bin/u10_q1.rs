use std::time::{Duration, Instant};

use async_lab::job_queue::{self, Job};

// 仕事の中身。broken は必ず失敗し、flaky は 2 回目まで失敗する
async fn perform(job: &Job) -> Result<(), String> {
    tokio::time::sleep(Duration::from_millis(20)).await;
    match job.payload.as_str() {
        "broken@example.com" => Err(String::from("宛先のサーバーが受け取りを拒否しました")),
        "flaky@example.com" if job.attempts < 3 => Err(String::from("時間切れ")),
        _ => Ok(()),
    }
}

// n 回目の失敗のあと、次にやり直すまでの時間: 100ms, 200ms, 400ms, ...
fn backoff(attempts: i64) -> Duration {
    Duration::from_millis(100 * 2u64.pow(attempts as u32 - 1))
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10retry");
    std::fs::create_dir_all("u10retry").expect("u10retry フォルダを作れません");
    let db = job_queue::open("u10retry/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    for to in ["ok@example.com", "flaky@example.com", "broken@example.com"] {
        job_queue::enqueue(&conn, "send_email", to, 5).await?;
    }

    let start = Instant::now();
    loop {
        match job_queue::claim(&conn, Duration::from_secs(30)).await? {
            Some(job) => match perform(&job).await {
                Ok(()) => {
                    job_queue::complete(&conn, job.id).await?;
                    println!(
                        "{:.2} 秒: #{} {} 成功（{} 回目）",
                        start.elapsed().as_secs_f64(),
                        job.id,
                        job.payload,
                        job.attempts
                    );
                }
                Err(error) => {
                    let status =
                        job_queue::fail(&conn, job.id, &error, backoff(job.attempts)).await?;
                    println!(
                        "{:.2} 秒: #{} {} 失敗（{} 回目）: {error} → {status}",
                        start.elapsed().as_secs_f64(),
                        job.id,
                        job.payload,
                        job.attempts
                    );
                }
            },
            None => {
                // 今すぐ実行できる仕事が無い。待っている仕事も無ければ終わり
                let counts = job_queue::counts(&conn).await?;
                if !counts.iter().any(|(status, _)| status == "queued") {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }
    }
    println!("最後の件数: {:?}", job_queue::counts(&conn).await?);

    // dead になった仕事は、理由と一緒に残っている（人が確かめて、直してから入れ直す）
    let mut rows = conn
        .query(
            "SELECT id, payload, attempts, last_error FROM jobs WHERE status = 'dead'",
            (),
        )
        .await?;
    while let Some(row) = rows.next().await? {
        println!(
            "dead: #{} {}（{} 回）最後のエラー: {}",
            row.get::<i64>(0)?,
            row.get::<String>(1)?,
            row.get::<i64>(2)?,
            row.get::<String>(3)?
        );
    }
    Ok(())
}
