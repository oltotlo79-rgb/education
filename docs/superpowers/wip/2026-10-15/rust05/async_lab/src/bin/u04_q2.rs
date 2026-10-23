use tokio::sync::{mpsc, oneshot};

// 係（アクター）への依頼
enum Command {
    Add {
        amount: u64,
        reply: oneshot::Sender<u64>,
    },
    // 合計を 0 に戻し、戻す前の合計を返事する
    Reset {
        reply: oneshot::Sender<u64>,
    },
}

async fn counter_actor(mut rx: mpsc::Receiver<Command>) {
    let mut total = 0;
    while let Some(command) = rx.recv().await {
        match command {
            Command::Add { amount, reply } => {
                total += amount;
                let _ = reply.send(total);
            }
            Command::Reset { reply } => {
                let old = total;
                total = 0;
                let _ = reply.send(old);
            }
        }
    }
}

// 依頼を送り、返事を待つ。返事の封筒は、依頼を作る関数 make で中に入れてもらう
async fn ask(
    tx: &mpsc::Sender<Command>,
    make: impl FnOnce(oneshot::Sender<u64>) -> Command,
) -> u64 {
    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(make(reply_tx)).await.expect("係がいません");
    reply_rx.await.expect("係が返事をしませんでした")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(16);
    tokio::spawn(counter_actor(rx));

    let total = ask(&tx, |reply| Command::Add { amount: 10, reply }).await;
    println!("10 を足した → 合計 {total}");
    let total = ask(&tx, |reply| Command::Add { amount: 20, reply }).await;
    println!("20 を足した → 合計 {total}");
    let old = ask(&tx, |reply| Command::Reset { reply }).await;
    println!("リセットした（それまでの合計 {old}）");
    let total = ask(&tx, |reply| Command::Add { amount: 5, reply }).await;
    println!("5 を足した → 合計 {total}");
}
