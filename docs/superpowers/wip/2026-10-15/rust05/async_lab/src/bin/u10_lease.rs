use std::time::{Duration, Instant};

use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10lease");
    std::fs::create_dir_all("u10lease").expect("u10lease フォルダを作れません");
    let db = job_queue::open("u10lease/jobs.db").await?;
    let worker_a = job_queue::connect(&db).await?;
    let worker_b = job_queue::connect(&db).await?;
    job_queue::enqueue(&worker_a, "send_email", "hanako@example.com", 3).await?;

    let start = Instant::now();
    // 持ち時間（lease）は 300ms
    let lease = Duration::from_millis(300);

    // ① worker A が仕事を取り出したところで、落ちてしまった（complete を呼ばない）
    let job = job_queue::claim(&worker_a, lease)
        .await?
        .expect("仕事がある");
    println!(
        "{:.1} 秒: A が #{} を取り出した（{} 回目）。ここで A が落ちた",
        start.elapsed().as_secs_f64(),
        job.id,
        job.attempts
    );

    // ② すぐに worker B が取りに来ても、A の持ち時間の間は取れない
    let job = job_queue::claim(&worker_b, lease).await?;
    println!(
        "{:.1} 秒: B が取りに来た → {:?}",
        start.elapsed().as_secs_f64(),
        job.map(|job| job.id)
    );

    // ③ 持ち時間が切れた後なら、B が取り出してやり直せる
    tokio::time::sleep(Duration::from_millis(400)).await;
    let job = job_queue::claim(&worker_b, lease)
        .await?
        .expect("仕事がある");
    println!(
        "{:.1} 秒: B が #{} を取り出した（{} 回目）",
        start.elapsed().as_secs_f64(),
        job.id,
        job.attempts
    );
    job_queue::complete(&worker_b, job.id).await?;
    println!("最後の件数: {:?}", job_queue::counts(&worker_b).await?);
    Ok(())
}
