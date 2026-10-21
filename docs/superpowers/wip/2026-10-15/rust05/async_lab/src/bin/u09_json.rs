use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;

#[instrument]
async fn delete_note(user_id: i64, note_id: i64) {
    debug!("DB から削除します");
    info!("ノートを削除しました");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // 1行1つの JSON で出す（ログを集めるサービスが読みやすい）
        .json()
        // RUST_LOG 環境変数で「どのくらい細かく出すか」を決める。無ければ info
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    delete_note(42, 7).await;
}
