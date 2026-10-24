use std::time::{Duration, Instant};
use tokio::time::timeout;

// 外部サービスの代わり: millis ミリ秒たってから答える
async fn call_service(name: &str, millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{name} の応答")
}

// 1つの呼び出しに、300ms の上限を付ける
async fn call_with_limit(name: &str, millis: u64) -> Result<String, String> {
    timeout(Duration::from_millis(300), call_service(name, millis))
        .await
        .map_err(|_| format!("{name} は時間切れ"))
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        call_with_limit("A", 100),
        call_with_limit("B", 400),
        call_with_limit("C", 200)
    );
    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
    println!("全体 {:.1} 秒", start.elapsed().as_secs_f64());
}
