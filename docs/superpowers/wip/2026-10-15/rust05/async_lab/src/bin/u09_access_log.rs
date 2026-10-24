use std::time::{Duration, Instant};

use tracing::{Instrument, info, info_span, instrument, warn};

// DB への問い合わせの代わり。スパンの中で呼ばれるので、ログにはリクエストの情報も付く
#[instrument]
async fn load_notes(user_id: i64) -> usize {
    tokio::time::sleep(Duration::from_millis(30)).await;
    info!("ノートを読みました");
    3
}

// 1つのリクエストの処理
async fn handle(path: &str, user_id: i64) -> u16 {
    let count = load_notes(user_id).await;
    if path == "/slow" {
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    if count > 0 { 200 } else { 404 }
}

// アクセスログ: リクエストごとにスパンを作り、終わったら1行にまとめて記録する
async fn serve(request_id: u64, method: &str, path: &str, user_id: i64) {
    let span = info_span!("request", request_id, method, path);
    async {
        let started = Instant::now();
        let status = handle(path, user_id).await;
        let elapsed_ms = started.elapsed().as_millis() as u64;
        if elapsed_ms >= 200 {
            warn!(status, elapsed_ms, "遅いリクエスト");
        } else {
            info!(status, elapsed_ms, "リクエスト完了");
        }
    }
    .instrument(span)
    .await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        // 時刻を出さない（本に載せる出力を短くするため。本番では出す）
        .without_time()
        .init();

    // 2つのリクエストが同時に来た
    tokio::join!(serve(1, "GET", "/notes", 42), serve(2, "GET", "/slow", 7));
}
