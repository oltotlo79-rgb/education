//! POST /notes/{id}/delete : ノートを削除し、303 で一覧へ。
//! 他人のノート・存在しないノートは 404。

use topcoat::{
    Result,
    context::Cx,
    router::{error::see_other, href, page, path_param},
};

use super::Id;
use crate::auth::require_user;
use crate::error::AppError;
use crate::flash::{self, Flash};
use crate::state::app_state;

#[page(POST)]
pub async fn delete(cx: &Cx) -> Result<()> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    app_state(cx)
        .notes
        .delete(user.id, id)
        .await
        .map_err(AppError::from)?;
    flash::set(cx, Flash::NoteDeleted);
    Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into())
}
