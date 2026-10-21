use std::time::Duration;

use tracing::{info, instrument, warn};
use tracing_subscriber::fmt::format::FmtSpan;

// #[instrument]: この関数を呼ぶたびに「スパン（処理の区間）」を作る。引数がフィールドとして記録される
#[instrument]
async fn find_user(user_id: i64) -> String {
    tokio::time::sleep(Duration::from_millis(20)).await;
    info!("ユーザーを見つけました");
    String::from("hanako")
}

#[instrument]
async fn search_notes(user_id: i64, query: &str) -> usize {
    tokio::time::sleep(Duration::from_millis(50)).await;
    let hits = 3;
    info!(hits, "検索しました");
    hits
}

// skip(password): パスワードはログに記録しない。fields で記録したい値を自分で足す
#[instrument(skip(password), fields(password_len = password.len()))]
async fn check_password(user_id: i64, password: &str) -> bool {
    tokio::time::sleep(Duration::from_millis(10)).await;
    warn!("この例では、どんなパスワードでも通します");
    true
}

// 1つのリクエストの処理。中で呼ぶ関数のスパンは、このスパンの「子」になる
#[instrument(fields(method = "GET"))]
async fn handle_request(path: &str, user_id: i64) {
    info!("リクエスト開始");
    check_password(user_id, "secret-pass").await;
    let name = find_user(user_id).await;
    let hits = search_notes(user_id, "買い物").await;
    info!(%name, hits, "リクエスト完了");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // スパンが閉じたとき（区間が終わったとき）にも1行出し、かかった時間を表示する
        .with_span_events(FmtSpan::CLOSE)
        // 時刻の代わりに「起動してからの秒数」を表示する（見やすくするため）
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    handle_request("/notes", 42).await;
}
