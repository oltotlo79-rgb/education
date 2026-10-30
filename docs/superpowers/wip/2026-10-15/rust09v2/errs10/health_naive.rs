//! GET /health : わざと間違えた版（DB に問い合わせない）。

use serde::Serialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Json, route},
};

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

#[route(GET)]
pub async fn health(_cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
    // DB には問い合わせず、いつも ok と答える
    Ok((StatusCode::OK, Json(Health { status: "ok" })))
}
