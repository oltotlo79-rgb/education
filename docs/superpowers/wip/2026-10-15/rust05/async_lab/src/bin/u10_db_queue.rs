use std::time::Duration;

use async_lab::job_queue;
use libsql::Connection;

// worker: 仕事を取り出して処理する。max_jobs 件やったら止まる（サーバーが止められた想定）
async fn run_worker(conn: &Connection, max_jobs: usize) -> Result<usize, libsql::Error> {
    let mut done = 0;
    while done < max_jobs {
        let Some(job) = job_queue::claim(conn, Duration::from_secs(30)).await? else {
            break;
        };
        // メールの送信の代わり
        tokio::time::sleep(Duration::from_millis(100)).await;
        job_queue::complete(conn, job.id).await?;
        println!("  #{} {} を処理しました", job.id, job.payload);
        done += 1;
    }
    Ok(done)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 何回動かしても同じ結果になるよう、前回の DB を消して作り直す
    let _ = std::fs::remove_dir_all("u10jobs");
    std::fs::create_dir_all("u10jobs").expect("u10jobs フォルダを作れません");

    // ① 1回目の起動: 5 件の仕事を入れ、2 件処理したところで止まる
    {
        let db = job_queue::open("u10jobs/jobs.db").await?;
        let conn = job_queue::connect(&db).await?;
        for n in 1..=5 {
            job_queue::enqueue(&conn, "send_email", &format!("メール{n}"), 3).await?;
        }
        println!("① 1回目の起動");
        run_worker(&conn, 2).await?;
        println!("  止める前の件数: {:?}", job_queue::counts(&conn).await?);
    }
    // ここで db と conn が片付けられる（プログラムを止めたのと同じ）

    // ② 2回目の起動: 同じ DB ファイルを開き直すと、残りの仕事を続きから処理できる
    let db = job_queue::open("u10jobs/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    println!("② 2回目の起動");
    println!(
        "  起動したときの件数: {:?}",
        job_queue::counts(&conn).await?
    );
    run_worker(&conn, usize::MAX).await?;
    println!("  最後の件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
