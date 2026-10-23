use std::time::{Duration, Instant};
use tokio::sync::watch;

// 止める合図（true）が来るまで、0.1 秒ごとに仕事をする
async fn worker(mut stop: watch::Receiver<bool>, start: Instant) -> u32 {
    let mut count = 0;
    loop {
        tokio::select! {
            changed = stop.changed() => {
                // 送り手がいなくなった（Err）か、値が true になったら止まる
                if changed.is_err() || *stop.borrow() {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                count += 1;
                println!("{:.1} 秒: 仕事 {count} 回目", start.elapsed().as_secs_f64());
            }
        }
    }
    count
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (stop_tx, stop_rx) = watch::channel(false);
    let handle = tokio::spawn(worker(stop_rx, start));

    tokio::time::sleep(Duration::from_millis(350)).await;
    println!(
        "{:.2} 秒: 止める合図を送ります",
        start.elapsed().as_secs_f64()
    );
    stop_tx.send(true).unwrap();

    let count = handle.await.unwrap();
    println!(
        "{:.2} 秒: 止まりました（仕事は {count} 回）",
        start.elapsed().as_secs_f64()
    );
}
