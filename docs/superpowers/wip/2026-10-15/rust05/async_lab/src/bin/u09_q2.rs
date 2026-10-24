use std::time::Duration;

use tracing::{info, instrument};

// 外の API のトークンはログに出さない。長さだけを記録する
#[instrument(skip(token), fields(token_len = token.len()))]
async fn call_translation_api(user_id: i64, text: &str, token: &str) -> String {
    tokio::time::sleep(Duration::from_millis(20)).await;
    info!("翻訳しました");
    format!("（訳）{text}")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().without_time().init();
    let translated = call_translation_api(42, "買い物メモ", "sk-very-secret-token-123").await;
    info!(%translated, "結果");
}
