//! GET /metrics : 動いている様子を数で返す（第11巻 UNIT 07 で、RustNote のコピーに足して試す）。
//!
//! Prometheus（数を集めて保存する道具）が読める、文字の形（text exposition format）で返す。
//! - rustnote_requests_total{class="2xx"} … 状態コードの種類ごとの、リクエストの数
//! - rustnote_request_duration_seconds … 応答にかかった時間の分布（ヒストグラム）

use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use topcoat::{Result, context::Cx, router::route};

/// 応答時間の区切り（秒）。「この秒数以下だった数」を数える
const BUCKETS: [f64; 6] = [0.005, 0.01, 0.05, 0.1, 0.5, 1.0];

/// 数えるための入れ物。どのスレッドからも同時に足せるよう、AtomicU64 にする
pub struct Metrics {
    by_class: [AtomicU64; 5],
    buckets: [AtomicU64; BUCKETS.len()],
    count: AtomicU64,
    sum_micros: AtomicU64,
}

/// アプリ全体で1つの入れ物（AtomicU64::new は const fn なので、static で作れる）
pub static METRICS: Metrics = Metrics {
    by_class: [const { AtomicU64::new(0) }; 5],
    buckets: [const { AtomicU64::new(0) }; BUCKETS.len()],
    count: AtomicU64::new(0),
    sum_micros: AtomicU64::new(0),
};

impl Metrics {
    /// 1つのリクエストの結果を数える
    pub fn record(&self, status: u16, elapsed: Duration) {
        let class = usize::from(status / 100).clamp(1, 5) - 1;
        self.by_class[class].fetch_add(1, Ordering::Relaxed);
        let seconds = elapsed.as_secs_f64();
        for (bucket, limit) in self.buckets.iter().zip(BUCKETS) {
            if seconds <= limit {
                bucket.fetch_add(1, Ordering::Relaxed);
            }
        }
        self.count.fetch_add(1, Ordering::Relaxed);
        let micros = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
        self.sum_micros.fetch_add(micros, Ordering::Relaxed);
    }

    /// Prometheus の文字の形にする
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# TYPE rustnote_requests_total counter\n");
        for (i, counter) in self.by_class.iter().enumerate() {
            let value = counter.load(Ordering::Relaxed);
            let _ = writeln!(out, "rustnote_requests_total{{class=\"{}xx\"}} {value}", i + 1);
        }
        out.push_str("# TYPE rustnote_request_duration_seconds histogram\n");
        for (bucket, limit) in self.buckets.iter().zip(BUCKETS) {
            let value = bucket.load(Ordering::Relaxed);
            let _ = writeln!(out, "rustnote_request_duration_seconds_bucket{{le=\"{limit}\"}} {value}");
        }
        let count = self.count.load(Ordering::Relaxed);
        let sum = self.sum_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        let _ = writeln!(out, "rustnote_request_duration_seconds_bucket{{le=\"+Inf\"}} {count}");
        let _ = writeln!(out, "rustnote_request_duration_seconds_sum {sum}");
        let _ = writeln!(out, "rustnote_request_duration_seconds_count {count}");
        out
    }
}

#[route(GET)]
pub async fn metrics(_cx: &Cx) -> Result<String> {
    Ok(METRICS.render())
}
