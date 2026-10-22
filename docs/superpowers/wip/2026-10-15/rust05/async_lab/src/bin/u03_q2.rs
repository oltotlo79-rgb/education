use std::time::{Duration, Instant};

// 遅い問い合わせ（0.8 秒かかる）
async fn slow_query() -> &'static str {
    tokio::time::sleep(Duration::from_millis(800)).await;
    "問い合わせの結果"
}

// 問い合わせと limit のタイマーを競争させ、タイマーが勝ったら時間切れにする
async fn query_with_limit(limit: Duration) -> Result<&'static str, String> {
    tokio::select! {
        value = slow_query() => Ok(value),
        _ = tokio::time::sleep(limit) => Err(format!("{} ミリ秒で時間切れ", limit.as_millis())),
    }
}

#[tokio::main]
async fn main() {
    for millis in [300, 1000] {
        let start = Instant::now();
        let result = query_with_limit(Duration::from_millis(millis)).await;
        println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
    }
}
