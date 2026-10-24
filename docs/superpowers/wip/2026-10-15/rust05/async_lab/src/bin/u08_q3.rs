use std::time::Duration;

// n 回目の失敗のあとに待つ時間（ばらつきなし）: 100ms から倍々、上限 2 秒
fn backoff_without_jitter(attempt: u32) -> Duration {
    let base = Duration::from_millis(100) * 2u32.pow(attempt - 1);
    base.min(Duration::from_secs(2))
}

fn main() {
    let mut total = Duration::ZERO;
    for attempt in 1..=6 {
        let wait = backoff_without_jitter(attempt);
        total += wait;
        println!(
            "{attempt} 回目の失敗のあと: {}ms 待つ（待った合計 {}ms）",
            wait.as_millis(),
            total.as_millis()
        );
    }
}
