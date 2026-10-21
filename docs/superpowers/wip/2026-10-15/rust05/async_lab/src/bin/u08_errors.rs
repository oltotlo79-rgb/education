use std::error::Error;
use std::time::{Duration, Instant};

use async_lab::mini_server::{self, Response};

// エラーの「原因の連なり」（source）を全部表示する
fn show_chain(label: &str, error: &reqwest::Error) {
    println!("{label}: {error}");
    let mut source = error.source();
    while let Some(cause) = source {
        println!("    原因: {cause}");
        source = cause.source();
    }
    println!(
        "    is_connect = {} / is_timeout = {}",
        error.is_connect(),
        error.is_timeout()
    );
}

#[tokio::main]
async fn main() {
    // ① 誰も待ち受けていないポートへ接続する
    let unused = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.local_addr().unwrap()
        // ここで listener が片付けられ、ポートは閉じる
    };
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let start = Instant::now();
    if let Err(error) = client.get(format!("http://{unused}/")).send().await {
        show_chain("① 接続できない", &error);
    }
    println!("    （{:.1} 秒かかった）", start.elapsed().as_secs_f64());

    // ② 答えが遅すぎるサーバー（1 秒かかる）に、300ms の時間切れで問い合わせる
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(300))
        .build()
        .unwrap();
    let slow = mini_server::start(|_| async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Response::new(200, "遅くなりました")
    })
    .await;
    if let Err(error) = client.get(format!("http://{slow}/")).send().await {
        show_chain("② 時間切れ", &error);
    }
}
