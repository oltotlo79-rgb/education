use std::pin::pin;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 全体の締め切り（1 秒後）。pin! で留めておくと、&mut で何度でも見張れる
    let mut deadline = pin!(tokio::time::sleep(Duration::from_secs(1)));
    let mut ticks = 0;
    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("{:.1} 秒: 締め切りです", start.elapsed().as_secs_f64());
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(300)) => {
                ticks += 1;
                println!("{:.1} 秒: {ticks} 回目の仕事", start.elapsed().as_secs_f64());
            }
        }
    }
}
