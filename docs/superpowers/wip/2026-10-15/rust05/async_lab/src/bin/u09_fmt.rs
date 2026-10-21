use tracing::{debug, error, info, warn};

fn main() {
    // ログの受け取り係（subscriber）を1つ決めて、プログラム全体に登録する
    tracing_subscriber::fmt().init();

    // 文章だけのログ
    info!("RustNote を起動しました");
    // 名前つきの値（フィールド）を添えたログ。あとで検索・集計しやすい
    info!(user_id = 42, note_id = 7, "ノートを作成しました");
    // ? を付けると Debug 表示、% を付けると Display 表示でフィールドに入れる
    let path = "/notes/7";
    let elapsed = std::time::Duration::from_millis(35);
    warn!(%path, ?elapsed, "応答が遅めです");
    error!(
        error = "DB に接続できません",
        "ヘルスチェックに失敗しました"
    );
    // 既定では INFO 以上だけが表示されるので、この行は出ない
    debug!("細かい調査用のログ");
}
