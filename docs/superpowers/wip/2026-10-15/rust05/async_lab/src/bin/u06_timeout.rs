use std::time::{Duration, Instant};
use tokio::time::timeout;

// 外部サービスの代わり: millis ミリ秒たってから答える
async fn call_service(millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{millis}ms で応答")
}

#[tokio::main]
async fn main() {
    for millis in [100, 800] {
        let start = Instant::now();
        // 最大 500ms だけ待つ。間に合えば Ok(結果)、間に合わなければ Err(Elapsed)
        match timeout(Duration::from_millis(500), call_service(millis)).await {
            Ok(answer) => println!("成功: {answer}（{:.1} 秒）", start.elapsed().as_secs_f64()),
            Err(elapsed) => println!(
                "時間切れ: {elapsed} / {elapsed:?}（{:.1} 秒）",
                start.elapsed().as_secs_f64()
            ),
        }
    }
}
