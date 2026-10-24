use std::time::Duration;
use tokio::time::timeout;
use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() {
    let tracker = TaskTracker::new();
    for id in 1..=2 {
        tracker.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("  タスク{id}: 終了");
        });
    }
    // tracker.close() を書き忘れた
    match timeout(Duration::from_secs(1), tracker.wait()).await {
        Ok(()) => println!("全員終わりました"),
        Err(_) => println!(
            "1 秒待っても wait が終わりません（残りのタスク: {}）",
            tracker.len()
        ),
    }
}
