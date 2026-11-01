//! /health … 動いているかの確かめ（onprem の 04 の 12 章の監視で開く）
//!
//! データベースにも聞いてみて、答えがあれば 200「ok」。無ければ 503。

use topcoat::{
    Result,
    context::Cx,
    router::{error::service_unavailable, route},
};

use crate::state::state;

#[route(GET)]
async fn health(cx: &Cx) -> Result<&'static str> {
    match sqlx::query("SELECT 1").execute(&state(cx).pool).await {
        Ok(_) => Ok("ok"),
        Err(e) => {
            eprintln!("health: データベースが答えません: {e}");
            Err(service_unavailable(30).into()) // 30 秒後にもう一度、と知らせる
        }
    }
}
