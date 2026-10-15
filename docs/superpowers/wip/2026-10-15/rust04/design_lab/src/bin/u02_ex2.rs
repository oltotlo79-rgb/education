// 「今の時刻（秒）」を教えてくれるものの約束
trait Clock {
    fn now(&self) -> u64;
}

// 本番用: OS の時計を読む
struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

// テスト用: 決めた時刻を返し続ける
struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

// セッションの期限切れ判定。時計の種類は知らない
fn is_expired(clock: &impl Clock, expires_at: u64) -> bool {
    clock.now() >= expires_at
}

fn main() {
    let before = FixedClock(999);
    let just = FixedClock(1000);
    println!("期限の1秒前: {}", is_expired(&before, 1000));
    println!("期限ちょうど: {}", is_expired(&just, 1000));
    // 本物の時計では、2000年1月1日(946684800)はとうに過ぎている
    println!("本物の時計: {}", is_expired(&SystemClock, 946_684_800));
}
