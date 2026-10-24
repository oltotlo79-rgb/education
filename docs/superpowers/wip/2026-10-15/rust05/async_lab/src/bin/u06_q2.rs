use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::timeout;

type Balances = Arc<Mutex<Vec<i64>>>;

// 例6-3 の悪い例と同じ: 「引く」と「足す」の間に .await がある
async fn transfer_unsafe(balances: Balances, from: usize, to: usize, amount: i64) {
    balances.lock().unwrap()[from] -= amount;
    println!("  {from} 番から {amount} 引いた");
    tokio::time::sleep(Duration::from_millis(300)).await;
    balances.lock().unwrap()[to] += amount;
    println!("  {to} 番に {amount} 足した");
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    // 仕事は別のタスクとして動かし、timeout は「待つこと」だけに付ける
    let handle = tokio::spawn(transfer_unsafe(Arc::clone(&balances), 0, 1, 300));
    match timeout(Duration::from_millis(100), handle).await {
        Ok(_) => println!("{:.1} 秒: 間に合いました", start.elapsed().as_secs_f64()),
        Err(_) => println!(
            "{:.1} 秒: 待つのはやめました（仕事は続いています）",
            start.elapsed().as_secs_f64()
        ),
    }
    // 少し後で残高を見る
    tokio::time::sleep(Duration::from_millis(400)).await;
    println!(
        "{:.1} 秒: 残高 {:?}",
        start.elapsed().as_secs_f64(),
        balances.lock().unwrap()
    );
}
