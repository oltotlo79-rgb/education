use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 箱の容量を 5 件に増やす
    let (tx, mut rx) = mpsc::channel::<u32>(5);

    // 遅い受け取り係: 1件の処理に 300ms かかる
    let consumer = tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            tokio::time::sleep(Duration::from_millis(300)).await;
            println!(
                "{:.1} 秒:   受け取り係 {job} 番を処理した",
                start.elapsed().as_secs_f64()
            );
        }
    });

    // 速い送り係: 5件を一気に送ろうとする
    for job in 1..=5 {
        // 箱がいっぱいなら、空きができるまでここで待たされる（= バックプレッシャー）
        tx.send(job).await.unwrap();
        println!(
            "{:.1} 秒: 送り係 {job} 番を箱に入れた",
            start.elapsed().as_secs_f64()
        );
    }
    drop(tx);
    consumer.await.unwrap();
}
