use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// 遅い処理（外部APIやディスクへの書き込み）の代わり
fn slow_work() {
    thread::sleep(Duration::from_millis(200));
}

fn run(hold_lock_while_working: bool) -> f64 {
    let log = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 0..3 {
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            if hold_lock_while_working {
                // 悪い例: 鍵を持ったまま遅い処理をする
                let mut guard = log.lock().unwrap();
                slow_work();
                guard.push(i);
            } else {
                // 良い例: 遅い処理は鍵の外で済ませ、鍵は一瞬だけ取る
                slow_work();
                log.lock().unwrap().push(i);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    start.elapsed().as_secs_f64()
}

fn main() {
    println!("鍵を持ったまま: 約 {:.1} 秒", run(true));
    println!("鍵の外で処理:   約 {:.1} 秒", run(false));
}
