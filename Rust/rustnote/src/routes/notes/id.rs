//! GET /notes/{id} : ノートの詳細。
//!
//! `module_param!` でこのモジュールの URL の区切りを `{id}` にし、
//! 文字列を `NoteId`（1 以上の数）に変換する。数字でない・0 以下なら 404。
//! 他人のノートも 404（「ある」こと自体を知らせないため、403 にはしない）。

pub mod delete;
pub mod edit;
pub mod toggle;

use topcoat::{
    Result,
    context::Cx,
    router::{href, module_param, page, path_param},
    view::{View, view},
};

use crate::auth::require_user;
use crate::domain::{Note, NoteId, UserId};
use crate::error::AppError;
use crate::flash;
use crate::state::app_state;
use crate::views::flash_banner;

// URL の {id} を NoteId に変換する。`Id` という型ができ、path_param::<Id>(cx) で読める。
// 変換に失敗したら NotFoundError（404）になる。
module_param!(pub id: NoteId, error = not_found);

/// 自分のノートを探す。無い・他人のノートなら AppError::NotFound（404）
pub async fn find_own_note(cx: &Cx, owner: UserId, id: NoteId) -> Result<Note, AppError> {
    app_state(cx)
        .notes
        .find(owner, id)
        .await?
        .ok_or(AppError::NotFound)
}

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    let note = find_own_note(cx, user.id, id).await?;
    let flash = flash::take(cx);

    Ok(view! {
        flash_banner(flash: flash)
        <article class=(if note.done { "note done" } else { "note" })>
            <h1>(note.title)</h1>
            <p class="meta">
                if note.done {
                    <span class="badge">"完了"</span>
                }
                "作成: "
                (note.created_at)
                " ／ 更新: "
                (note.updated_at)
            </p>
            // 本文は自動でエスケープされる。改行は CSS（white-space: pre-wrap）で表示する
            <div class="note-body">(note.body)</div>
        </article>

        <div class="note-actions">
            <a class="button" href=(href!(edit::edit_page, Id(id)))>"編集"</a>
            <form method="post" action=(href!(toggle::toggle, Id(id)))>
                <button type="submit" class="secondary">
                    if note.done {
                        "未完了に戻す"
                    } else {
                        "完了にする"
                    }
                </button>
            </form>
            <form method="post" action=(href!(delete::delete, Id(id)))>
                <button type="submit" class="danger">"削除"</button>
            </form>
        </div>

        <p><a href=(href!(super::index))>"← 一覧に戻る"</a></p>
    })
}
