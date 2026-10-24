use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

// アプリ全体で共有する「数え係」。数を足すだけなので、Mutex ではなく原子的な整数を使う
#[derive(Default)]
struct Metrics {
    requests: AtomicU64,
    errors: AtomicU64,
    // 応答時間の分布（ヒストグラム）: 50ms 未満 / 200ms 未満 / それ以上
    fast: AtomicU64,
    normal: AtomicU64,
    slow: AtomicU64,
}

impl Metrics {
    fn record(&self, elapsed: Duration, ok: bool) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        if !ok {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
        let bucket = if elapsed < Duration::from_millis(50) {
            &self.fast
        } else if elapsed < Duration::from_millis(200) {
            &self.normal
        } else {
            &self.slow
        };
        bucket.fetch_add(1, Ordering::Relaxed);
    }

    // Prometheus（メトリクスを集める定番のソフト）が読める文字の形で書き出す
    fn render(&self) -> String {
        format!(
            "http_requests_total {}\nhttp_errors_total {}\nhttp_duration_bucket{{le=\"0.05\"}} {}\nhttp_duration_bucket{{le=\"0.2\"}} {}\nhttp_duration_bucket{{le=\"+Inf\"}} {}",
            self.requests.load(Ordering::Relaxed),
            self.errors.load(Ordering::Relaxed),
            self.fast.load(Ordering::Relaxed),
            self.fast.load(Ordering::Relaxed) + self.normal.load(Ordering::Relaxed),
            self.requests.load(Ordering::Relaxed),
        )
    }
}

// リクエストの処理の代わり。番号によって速さと成否が変わる
async fn handle(n: u64) -> bool {
    let millis = [10, 30, 120, 250][(n % 4) as usize];
    tokio::time::sleep(Duration::from_millis(millis)).await;
    !n.is_multiple_of(5)
}

#[tokio::main]
async fn main() {
    let metrics = Arc::new(Metrics::default());
    let mut handles = Vec::new();
    for n in 1..=20 {
        let metrics = Arc::clone(&metrics);
        handles.push(tokio::spawn(async move {
            let start = Instant::now();
            let ok = handle(n).await;
            metrics.record(start.elapsed(), ok);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("{}", metrics.render());
}
