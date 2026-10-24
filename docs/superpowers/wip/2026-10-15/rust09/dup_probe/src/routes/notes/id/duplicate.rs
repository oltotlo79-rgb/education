//! POST /notes/{id}/duplicate : 自分のノートを複製し、303 で新しいノートの詳細へ移る。
//! 他人のノート・存在しないノートは 404。

use topcoat::{
    Result,
    context::Cx,
    router::{error::see_other, href, page, path_param},
};

use super::{Id, find_own_note};
use crate::auth::require_user;
use crate::domain::NewNote;
use crate::error::AppError;
use crate::flash::{self, Flash};
use crate::state::app_state;

#[page(POST)]
pub async fn duplicate(cx: &Cx) -> Result<()> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    // 他人のノートなら、ここで 404 になる
    let note = find_own_note(cx, user.id, id).await?;
    // DB から読んだタイトルと本文も、型の検査を通してから保存する。
    // 保存済みの値が検査に落ちるのは DB の異常なので、500 にする
    let copy = NewNote::parse(&note.title, &note.body)
        .map_err(|errors| AppError::internal(format!("{errors:?}")))?;
    let created = app_state(cx)
        .notes
        .insert(user.id, &copy)
        .await
        .map_err(AppError::from)?;
    flash::set(cx, Flash::NoteCreated);
    Err(see_other(href!(super::show, Id(created.id)).resolve(cx)).into())
}
