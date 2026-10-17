//! GET /notes/{id}/edit  : 編集フォーム（今の version を hidden で埋め込む）
//! POST /notes/{id}/edit : 保存して 303 で詳細へ
//!
//! 楽観的ロック: フォームを開いたときの version と DB の version が違えば、
//! ほかの画面（別のタブなど）で先に保存されたということ。上書きせずに 409 で知らせる。
//!
//! | 結果 | ステータス |
//! |---|---|
//! | 成功 | 303 → /notes/{id}（フラッシュ「保存しました」） |
//! | タイトルが空・長すぎ | 400（入力を残してフォームを再表示） |
//! | version が古い | 409（最新の version でフォームを再表示。もう一度保存すると上書き） |
//! | 他人のノート・存在しない | 404 |

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page, path_param},
    view::{View, view},
};

use super::{Id, find_own_note};
use crate::auth::require_user;
use crate::domain::NewNote;
use crate::error::{AppError, RepoError};
use crate::flash::{self, Flash};
use crate::state::app_state;
use crate::views::{NoteFormState, note_form};

/// 編集フォームから送られてくる値
#[derive(Deserialize)]
pub struct EditForm {
    title: String,
    body: String,
    /// フォームを開いたときの version
    version: i64,
}

#[page]
pub async fn edit_page(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    let note = find_own_note(cx, user.id, id).await?;

    let form = NoteFormState {
        version: Some(note.version),
        ..NoteFormState::from_input(&note.title, &note.body)
    };
    Ok(view! {
        <h1>"ノートを編集"</h1>
        note_form(
            action: href!(update, Id(id)).resolve(cx),
            submit_label: "保存する",
            form: form
        )
        <p><a href=(href!(super::show, Id(id)))>"← 保存せずに戻る"</a></p>
    })
}

#[page(POST)]
pub async fn update(cx: &Cx, Form(input): Form<EditForm>) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    // 他人のノート・存在しないノートは、入力の検査より先に 404 にする
    find_own_note(cx, user.id, id).await?;

    let mut form = NoteFormState {
        version: Some(input.version),
        ..NoteFormState::from_input(&input.title, &input.body)
    };

    let status = match NewNote::parse(&input.title, &input.body) {
        Err(errors) => {
            form = form.with_errors(&errors);
            StatusCode::BAD_REQUEST
        }
        Ok(new_note) => {
            let result = app_state(cx)
                .notes
                .update(user.id, id, &new_note, input.version)
                .await;
            match result {
                Ok(_) => {
                    flash::set(cx, Flash::NoteUpdated);
                    return Err(see_other(href!(super::show, Id(id)).resolve(cx)).into());
                }
                Err(RepoError::VersionMismatch) => {
                    // 最新の内容を読み直し、その version で再表示する（入力した内容は残す）
                    let latest = find_own_note(cx, user.id, id).await?;
                    form.version = Some(latest.version);
                    form.notice = Some(format!(
                        "{}。保存されている最新のタイトルは「{}」です。この内容で上書きする場合は、もう一度「保存する」を押してください。",
                        AppError::Conflict,
                        latest.title
                    ));
                    StatusCode::CONFLICT
                }
                Err(other) => return Err(AppError::from(other).into()),
            }
        }
    };

    Ok(view! {
        (status)
        <h1>"ノートを編集"</h1>
        note_form(
            action: href!(update, Id(id)).resolve(cx),
            submit_label: "保存する",
            form: form
        )
        <p><a href=(href!(super::show, Id(id)))>"← 保存せずに戻る"</a></p>
    })
}
