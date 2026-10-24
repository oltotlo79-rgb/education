use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10two");
    std::fs::create_dir_all("u10two").expect("u10two フォルダを作れません");
    let db = Arc::new(job_queue::open("u10two/jobs.db").await?);
    let conn = job_queue::connect(&db).await?;
    for n in 1..=10 {
        job_queue::enqueue(&conn, "make_thumbnail", &format!("画像{n}"), 3).await?;
    }

    // worker の数は設定（環境変数 WORKERS）で決める。無ければ 2
    let workers: usize = std::env::var("WORKERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2);
    // だれがどの仕事をしたかの記録
    let log = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for number in 1..=workers {
        let db = Arc::clone(&db);
        let log = Arc::clone(&log);
        handles.push(tokio::spawn(async move {
            // worker ごとに自分の接続を使う
            let conn = job_queue::connect(&db).await?;
            while let Some(job) = job_queue::claim(&conn, Duration::from_secs(30)).await? {
                tokio::time::sleep(Duration::from_millis(50)).await;
                job_queue::complete(&conn, job.id).await?;
                log.lock().unwrap().push((job.id, number));
            }
            Ok::<(), libsql::Error>(())
        }));
    }
    for handle in handles {
        handle.await.expect("worker が panic しました")?;
    }

    let mut log = log.lock().unwrap().clone();
    log.sort();
    for number in 1..=workers {
        let ids: Vec<i64> = log
            .iter()
            .filter(|(_, worker)| *worker == number)
            .map(|(id, _)| *id)
            .collect();
        println!("worker{number}: {} 件 {:?}", ids.len(), ids);
    }
    let mut ids: Vec<i64> = log.iter().map(|(id, _)| *id).collect();
    let total = ids.len();
    ids.dedup();
    println!(
        "処理した回数 {total} / 仕事の数 {} / 重複 {} 件",
        ids.len(),
        total - ids.len()
    );
    println!("最後の件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
