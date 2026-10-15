//! GET /api/health -> {"status":"ok"}

use serde::Serialize;
use topcoat::{
    Result,
    router::{content::Json, route},
};

#[derive(Serialize)]
pub(crate) struct Health {
    status: &'static str,
}

#[route(GET)]
pub(crate) async fn health() -> Result<Json<Health>> {
    Ok(Json(Health { status: "ok" }))
}
