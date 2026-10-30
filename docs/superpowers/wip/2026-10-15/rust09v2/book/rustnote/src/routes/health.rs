//! GET /health : 動いているかの確認（Fly.io のヘルスチェックが使う）。
//!
//! DB に `SELECT 1` を問い合わせ、成功なら 200 {"status":"ok"}、
//! 失敗なら 503 {"status":"unavailable"} を返す。

use serde::Serialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Json, route},
};

use crate::state::app_state;

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

#[route(GET)]
pub async fn health(cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
    match app_state(cx).db.ping().await {
        Ok(()) => Ok((StatusCode::OK, Json(Health { status: "ok" }))),
        Err(error) => {
            eprintln!("[error] ヘルスチェックで DB に接続できません: {error}");
            Ok((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(Health {
                    status: "unavailable",
                }),
            ))
        }
    }
}
