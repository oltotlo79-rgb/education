use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8);

    let worker = tokio::spawn(async move {
        let mut saved = 0;
        while let Some(title) = rx.recv().await {
            println!("  受け取り: {title}");
            saved += 1;
        }
        saved
    });

    let sender = {
        let tx = tx.clone();
        tokio::spawn(async move {
            for n in 1..=2 {
                tx.send(format!("A-{n}")).await.unwrap();
            }
        })
    };
    sender.await.unwrap();
    // drop(tx) を書き忘れた。main の tx が残っているので、箱は閉じない

    // 受け取り係の終わりを、1 秒だけ待ってみる
    tokio::select! {
        saved = worker => println!("保存した件数: {}", saved.unwrap()),
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
            println!("1 秒待っても、受け取り係が終わりません");
        }
    }
}
