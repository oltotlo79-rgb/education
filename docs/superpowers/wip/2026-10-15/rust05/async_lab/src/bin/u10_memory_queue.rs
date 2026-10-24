use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // メモリの中の仕事の列（UNIT 04 の mpsc）
    let (tx, mut rx) = mpsc::channel::<String>(100);
    for n in 1..=5 {
        tx.send(format!("メール{n}")).await.unwrap();
    }
    println!("5 件の仕事を列に入れました");

    let done = Arc::new(AtomicU32::new(0));
    let worker_done = Arc::clone(&done);
    let worker = tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            // 1件に 300ms かかる（メールの送信の代わり）
            tokio::time::sleep(Duration::from_millis(300)).await;
            worker_done.fetch_add(1, Ordering::Relaxed);
            println!(
                "{:.1} 秒: {job} を送りました",
                start.elapsed().as_secs_f64()
            );
        }
    });

    // 0.7 秒後に、サーバーの入れ替え（デプロイ）で止められたとする
    tokio::time::sleep(Duration::from_millis(700)).await;
    println!(
        "{:.1} 秒: サーバーを止めます",
        start.elapsed().as_secs_f64()
    );
    worker.abort();

    let sent = done.load(Ordering::Relaxed);
    println!(
        "送れた仕事: {sent} 件 / メモリと一緒に消えた仕事: {} 件",
        5 - sent
    );
}
