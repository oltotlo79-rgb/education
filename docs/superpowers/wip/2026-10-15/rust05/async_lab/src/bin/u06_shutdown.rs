use std::time::Duration;
use tokio::time::{interval, sleep, timeout};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

// 裏で動き続ける係。0.5 秒ごとに仕事をし、停止の合図が来たら片付けて終わる
async fn worker(id: u32, token: CancellationToken) {
    let mut ticker = interval(Duration::from_millis(500));
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("  worker{id}: 停止の合図を受け取った。片付けます");
                // 片付けにも少し時間がかかる（書きかけのデータを保存するなど）
                sleep(Duration::from_millis(200)).await;
                println!("  worker{id}: 終了");
                return;
            }
            _ = ticker.tick() => {
                println!("  worker{id}: 仕事をしました");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // 停止の合図を全員に配るための札
    let token = CancellationToken::new();
    // 作ったタスクを覚えておき、全部終わるのを待てるようにする係
    let tracker = TaskTracker::new();
    for id in 1..=2 {
        tracker.spawn(worker(id, token.child_token()));
    }

    println!("起動しました。Ctrl+C で止まります");
    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl+C を待ち受けられません");
    println!("Ctrl+C を受け取りました。停止を始めます");

    // ① もう新しいタスクは受け付けない
    tracker.close();
    // ② 全員に停止の合図を出す
    token.cancel();
    // ③ 最大 5 秒だけ、全員が終わるのを待つ
    match timeout(Duration::from_secs(5), tracker.wait()).await {
        Ok(()) => println!("全員きれいに終わりました"),
        Err(_) => println!("5 秒待っても終わらないタスクがあるので、打ち切ります"),
    }
}
