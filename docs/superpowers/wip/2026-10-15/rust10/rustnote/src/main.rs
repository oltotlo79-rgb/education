//! RustNote の実行ファイルの入口。
//!
//! 起動の流れ:
//! 1. .env があれば環境変数として読み込む（開発用。本番では使わない）
//! 2. 設定（Config）を環境変数から作る
//! 3. アセット（CSS）の束を読み込む
//! 4. DB を開き、まだ適用していないマイグレーションを適用する
//! 5. ルーターを組み立て、HOST:PORT（既定 127.0.0.1:3000）で待ち受ける
//!
//! 起動時の失敗は anyhow で「何をしようとして失敗したか」を付けて表示し、終了する。

use anyhow::Context;
use rustnote::{
    config::{self, Config},
    db::{self, Db},
    routes,
    state::AppState,
};
use topcoat::asset::AssetBundle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env が無くてもエラーにしない（本番では環境変数を直接設定する）
    dotenvy::dotenv().ok();

    let config = Config::from_env().context("設定を読み込めませんでした")?;

    // `topcoat asset bundle`（または `topcoat dev`）が実行ファイルの隣に作る assets/ を読む。
    // 無ければ DB に触る前に止める
    let assets = AssetBundle::load().context(
        "アセットの束（assets/）がありません。先に `topcoat asset bundle` を実行するか、`topcoat dev` で起動してください",
    )?;

    let db = Db::open(&config)
        .await
        .with_context(|| format!("DB（{}）を開けませんでした", config.database_url))?;
    let applied = db::migrate(&db)
        .await
        .context("マイグレーションに失敗しました")?;
    for version in &applied {
        eprintln!("マイグレーションを適用しました: {version}");
    }

    let router = routes::build_router(AppState::new(db), assets);

    eprintln!(
        "RustNote を起動しました: http://{}",
        config::listen_address()
    );
    topcoat::start(router)
        .await
        .context("サーバーを起動できませんでした（HOST / PORT を確認してください）")?;
    Ok(())
}
