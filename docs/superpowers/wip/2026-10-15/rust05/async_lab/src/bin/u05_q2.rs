use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

// DB への問い合わせの代わり。300ms かかる
async fn query(n: u32, start: Instant) {
    println!(
        "{:.1} 秒: {n} 番 問い合わせ開始",
        start.elapsed().as_secs_f64()
    );
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 同時に使える DB の接続は 3 本まで
    let connections = Arc::new(Semaphore::new(3));
    let mut handles = Vec::new();
    for n in 1..=6 {
        let connections = Arc::clone(&connections);
        handles.push(tokio::spawn(async move {
            let _permit = connections.acquire_owned().await.unwrap();
            query(n, start).await;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!(
        "全部終わりました（{:.1} 秒）",
        start.elapsed().as_secs_f64()
    );
}
