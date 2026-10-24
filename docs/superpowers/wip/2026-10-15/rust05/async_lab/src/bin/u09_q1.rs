use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

fn main() {
    // RUST_LOG 環境変数で細かさを決められるようにする。無ければ info
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("RustNote を起動しました");
    info!(user_id = 42, note_id = 7, "ノートを作成しました");
    let path = "/notes/7";
    let elapsed = std::time::Duration::from_millis(35);
    warn!(%path, ?elapsed, "応答が遅めです");
    error!(
        error = "DB に接続できません",
        "ヘルスチェックに失敗しました"
    );
    debug!("細かい調査用のログ");
}
