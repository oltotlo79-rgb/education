use std::pin::pin;
use std::time::{Duration, Instant};

// 0.5 秒かかる仕事
async fn long_job() -> &'static str {
    println!("  long_job: 始めます");
    tokio::time::sleep(Duration::from_millis(500)).await;
    "仕事の結果"
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 仕事の Future はループの外で1回だけ作り、pin! で留める
    let mut job = pin!(long_job());
    for round in 1..=4 {
        tokio::select! {
            // &mut で借りて進める。負けても捨てられず、次のループで続きから進む
            result = &mut job => {
                println!("{:.1} 秒: {result}", start.elapsed().as_secs_f64());
                return;
            }
            _ = tokio::time::sleep(Duration::from_millis(200)) => {
                println!("{:.1} 秒: 見回り {round} 回目", start.elapsed().as_secs_f64());
            }
        }
    }
    println!(
        "{:.1} 秒: 仕事は一度も終わりませんでした",
        start.elapsed().as_secs_f64()
    );
}
