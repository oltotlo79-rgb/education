use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 100ms で打ち切るつもりで、中で「スレッドごと眠る」処理をしてしまう
    let result = timeout(Duration::from_millis(100), async {
        std::thread::sleep(Duration::from_secs(1));
        "1 秒の処理が終わった"
    })
    .await;
    println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
}
