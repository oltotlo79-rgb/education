//! 第8巻 UNIT 04: ログインの回数の制限の、いちばん小さな形（RustNote には入っていない）。
//! 「決まった時間の窓の中で、同じ鍵（メールアドレスや送り元）の失敗が上限を超えたら断る」を、
//! 時計を自分で進めながら試す。
//! 実行: cargo run -q --example u04_limiter

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 鍵ごとに、窓の始まりと、窓の中の失敗の回数を覚える
struct Limiter {
    window: Duration,
    max_failures: u32,
    seen: HashMap<String, (Instant, u32)>,
}

impl Limiter {
    fn new(window: Duration, max_failures: u32) -> Limiter {
        Limiter {
            window,
            max_failures,
            seen: HashMap::new(),
        }
    }

    /// 今、この鍵で試してよいか（窓が過ぎていれば、数え直す）
    fn allowed(&mut self, key: &str, now: Instant) -> bool {
        match self.seen.get(key) {
            Some((start, count)) if now.duration_since(*start) < self.window => {
                *count < self.max_failures
            }
            _ => true,
        }
    }

    /// 失敗を1回数える
    fn record_failure(&mut self, key: &str, now: Instant) {
        let entry = self.seen.entry(key.to_string()).or_insert((now, 0));
        if now.duration_since(entry.0) >= self.window {
            *entry = (now, 0);
        }
        entry.1 += 1;
    }
}

fn main() {
    let start = Instant::now();
    let at = |minutes: u64| start + Duration::from_secs(minutes * 60);
    // 10 分の窓で、5 回まで
    let mut by_email = Limiter::new(Duration::from_secs(600), 5);

    println!("--- 1つのメール（alice）を、続けて試す");
    for n in 1..=7 {
        let ok = by_email.allowed("alice@example.com", at(n));
        println!(
            "{n} 分目: {}",
            if ok {
                "試せる（失敗）"
            } else {
                "断る"
            }
        );
        if ok {
            by_email.record_failure("alice@example.com", at(n));
        }
    }
    let ok = by_email.allowed("alice@example.com", at(8));
    println!(
        "8 分目（本人が正しいパスワードで）: {}",
        if ok { "試せる" } else { "断る" }
    );
    let ok = by_email.allowed("alice@example.com", at(12));
    println!(
        "12 分目（本人が正しいパスワードで）: {}",
        if ok { "試せる" } else { "断る" }
    );

    println!("--- 1つのパスワードを、7人のメールで1回ずつ試す");
    let mut by_email = Limiter::new(Duration::from_secs(600), 5);
    let mut by_source = Limiter::new(Duration::from_secs(600), 5);
    for n in 1..=7 {
        let email = format!("user{n}@example.com");
        let source = "203.0.113.7";
        let email_ok = by_email.allowed(&email, at(1));
        let source_ok = by_source.allowed(source, at(1));
        println!(
            "{email}: メールごとの制限 {}・送り元ごとの制限 {}",
            if email_ok { "試せる" } else { "断る" },
            if source_ok { "試せる" } else { "断る" }
        );
        by_email.record_failure(&email, at(1));
        by_source.record_failure(source, at(1));
    }
}
