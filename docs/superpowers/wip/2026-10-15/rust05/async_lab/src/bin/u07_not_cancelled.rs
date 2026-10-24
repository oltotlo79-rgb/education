use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 1 秒かかる同期の処理を、別のスレッドで動かす
    let handle = tokio::task::spawn_blocking(move || {
        std::thread::sleep(Duration::from_secs(1));
        println!(
            "{:.1} 秒: 別スレッドの処理が最後まで終わった",
            start.elapsed().as_secs_f64()
        );
    });
    // 待つのは 0.1 秒まで
    match timeout(Duration::from_millis(100), handle).await {
        Ok(_) => println!("間に合いました"),
        Err(_) => println!(
            "{:.1} 秒: 待つのをやめました",
            start.elapsed().as_secs_f64()
        ),
    }
    println!("{:.1} 秒: main が終わります", start.elapsed().as_secs_f64());
}
