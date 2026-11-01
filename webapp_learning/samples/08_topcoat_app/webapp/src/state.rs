//! アプリ全体で 1 つだけ持つ物（データベースの接続のたまり場と、設定）
//!
//! ルーターに `.app_context(AppState { .. })` で渡し、どのページからも `state(cx)` で取り出す。

use std::sync::Arc;

use sqlx::{PgPool, postgres::PgPoolOptions};
use topcoat::context::{Cx, app_context};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    /// 接続のたまり場。使うたびに、空いている接続を 1 つ借りて、返す
    pub pool: PgPool,
    pub config: Arc<Config>,
}

/// どのページ・どの関数からでも、アプリの共有の物を取り出す
pub fn state(cx: &Cx) -> &AppState {
    app_context::<AppState>(cx)
}

/// PostgreSQL につなぐ。つながらなければ、5 秒であきらめてエラーにする
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}

/// migrations フォルダの SQL のうち、まだ流していない物を、番号の順に流す
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
