//! POST /notes/{id}/toggle : 完了（done）と未完了を切り替え、303 で詳細へ戻る。
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

// 成功すると必ず 303 なので、画面として返す中身は無い（`()`）
#[page(POST)]
pub async fn toggle(cx: &Cx) -> Result<()> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    // 見つからなければ RepoError::NotFound → AppError::NotFound（404）
    app_state(cx)
        .notes
        .toggle_done(user.id, id)
        .await
        .map_err(AppError::from)?;
    flash::set(cx, Flash::NoteToggled);
    Err(see_other(href!(super::show, Id(id)).resolve(cx)).into())
}
