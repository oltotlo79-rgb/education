//! GET /api/notes (JSON list) and POST /api/notes (JSON body -> 201 + JSON).
//! Unauthenticated requests get 401 (ok_or_unauthorized).

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Json, error::RouterErrorExt, route},
};

use crate::app::{Note, current_user, db};

#[route(GET)]
pub(crate) async fn list(cx: &Cx) -> Result<Json<Vec<Note>>> {
    let user = current_user(cx).await.ok_or_unauthorized()?;
    Ok(Json(db(cx).list_notes(&user.name, None)))
}

#[derive(Deserialize)]
pub(crate) struct NewNote {
    title: String,
    #[serde(default)]
    body: String,
}

#[route(POST)]
pub(crate) async fn create(
    cx: &Cx,
    Json(input): Json<NewNote>,
) -> Result<(StatusCode, Json<Note>)> {
    let user = current_user(cx).await.ok_or_unauthorized()?;
    let id = db(cx).create_note(&user.name, input.title, input.body);
    let note = db(cx).get_note(id).ok_or_not_found()?;
    Ok((StatusCode::CREATED, Json(note)))
}
