use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;

// 口座の残高（共有）
type Balances = Arc<Mutex<Vec<i64>>>;

// ① 悪い例: 「引く」と「足す」の間に .await がある
async fn transfer_unsafe(balances: Balances, from: usize, to: usize, amount: i64) {
    balances.lock().unwrap()[from] -= amount;
    println!("  {from} 番から {amount} 引いた");
    // 相手の銀行へ連絡する（時間がかかる）
    tokio::time::sleep(Duration::from_millis(300)).await;
    balances.lock().unwrap()[to] += amount;
    println!("  {to} 番に {amount} 足した");
}

// ② 良い例: 時間のかかる連絡を先に済ませ、残高の変更は .await をはさまずに一気に行う
async fn transfer_safe(balances: Balances, from: usize, to: usize, amount: i64) {
    tokio::time::sleep(Duration::from_millis(300)).await;
    let mut b = balances.lock().unwrap();
    b[from] -= amount;
    b[to] += amount;
    println!("  {from} 番 → {to} 番へ {amount} 移した");
}

#[tokio::main]
async fn main() {
    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    // 100ms で打ち切る（= 途中で Future が片付けられる）
    let result = timeout(
        Duration::from_millis(100),
        transfer_unsafe(Arc::clone(&balances), 0, 1, 300),
    )
    .await;
    println!("① {result:?} → 残高 {:?}", balances.lock().unwrap());

    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    let result = timeout(
        Duration::from_millis(100),
        transfer_safe(Arc::clone(&balances), 0, 1, 300),
    )
    .await;
    println!("② {result:?} → 残高 {:?}", balances.lock().unwrap());
}
