use tokio::sync::mpsc::{self, error::TrySendError};

#[tokio::main]
async fn main() {
    // 箱の容量は 2 件
    let (tx, mut rx) = mpsc::channel::<u32>(2);

    // 受け取り係が動き出す前に、5 件の依頼が一気に来たとする
    for job in 1..=5 {
        // try_send は待たない。箱がいっぱいなら、すぐに Full のエラーを返す
        match tx.try_send(job) {
            Ok(()) => println!("{job} 番: 受け付けました"),
            Err(TrySendError::Full(job)) => {
                println!("{job} 番: 混んでいるので断りました（503 を返す）");
            }
            Err(TrySendError::Closed(job)) => println!("{job} 番: 受け取り係がいません"),
        }
    }
    drop(tx);

    // 受け付けた分だけを処理する
    while let Some(job) = rx.recv().await {
        println!("受け取り係: {job} 番を処理しました");
    }
}
