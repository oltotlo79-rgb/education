//! GET /notes  : 自分のノートの一覧（?q= でキーワード検索、?page= でページ送り）と作成フォーム
//! POST /notes : ノートを作成し、303 でそのノートの詳細へ（入力に問題があれば 400 で再表示）
//!
//! どちらもログインが必要（未ログインなら 303 でログイン画面へ）。

pub mod id;

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page, query_params},
    view::{View, class, component, view},
};

use crate::auth::require_user;
use crate::domain::{NewNote, NoteQuery, UserId};
use crate::error::AppError;
use crate::flash::{self, Flash};
use crate::state::app_state;
use crate::views::{NoteFormState, flash_banner, note_form, pagination};

/// URL のクエリ（?q=...&page=...）。page が数字でなければ 400
#[query_params(error = bad_request)]
pub struct NotesParams {
    q: Option<String>,
    page: Option<u32>,
}

/// 作成フォームから送られてくる値
#[derive(Deserialize)]
pub struct NoteForm {
    pub title: String,
    pub body: String,
}

/// 一覧の本体（検索フォーム・作成フォーム・ノートの一覧・ページ送り）。
/// コンポーネントは async なので、この中で DB を検索できる。
#[component]
async fn notes_index(
    cx: &Cx,
    owner: UserId,
    query: NoteQuery,
    form: NoteFormState,
) -> Result<impl View> {
    let page = app_state(cx)
        .notes
        .search(owner, &query)
        .await
        .map_err(AppError::from)?;
    let keyword = query.keyword.clone();

    Ok(view! {
        <h1>"ノート"</h1>

        <form method="get" action=(href!(index)) class="search-form">
            <input
                name="q"
                type="search"
                placeholder="キーワードで検索"
                value=(keyword.as_deref())
            >
            <button type="submit">"検索"</button>
        </form>

        <details
            class="new-note"
            open=(form.title_error.is_some() || form.body_error.is_some())
        >
            <summary>"新しいノートを書く"</summary>
            note_form(
                action: href!(create).resolve(cx),
                submit_label: "作成する",
                form: form
            )
        </details>

        if page.notes.is_empty() {
            <p class="empty">
                if keyword.is_some() {
                    "条件に合うノートはありません。"
                } else {
                    "まだノートがありません。最初の1件を書いてみましょう。"
                }
            </p>
        } else {
            <ul class="note-list">
                #[key(note.id.get())]
                for note in page.notes.clone() {
                    <li class=(class!("note-item", "done" if note.done))>
                        <a href=(href!(id::show, id::Id(note.id)))>(note.title)</a>
                        if note.done {
                            <span class="badge">"完了"</span>
                        }
                        <span class="updated">
                            "更新: "
                            (note.updated_at)
                        </span>
                    </li>
                }
            </ul>
        }

        pagination(page: page, query: query)
    })
}

#[page]
pub async fn index(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let params = query_params::<NotesParams>(cx)?;
    let query = NoteQuery::new(params.q.as_deref(), params.page);
    let flash = flash::take(cx);

    Ok(view! {
        flash_banner(flash: flash)
        notes_index(owner: user.id, query: query, form: NoteFormState::default())
    })
}

#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;

    let errors = match NewNote::parse(&input.title, &input.body) {
        Ok(new_note) => {
            let note = app_state(cx)
                .notes
                .insert(user.id, &new_note)
                .await
                .map_err(AppError::from)?;
            flash::set(cx, Flash::NoteCreated);
            // PRG: 作成できたら 303 で詳細ページ（GET）へ
            return Err(see_other(href!(id::show, id::Id(note.id)).resolve(cx)).into());
        }
        Err(errors) => errors,
    };

    // 入力に問題があった: 400 で一覧と入力済みのフォームを再表示する
    let form = NoteFormState::from_input(&input.title, &input.body).with_errors(&errors);
    Ok(view! {
        (StatusCode::BAD_REQUEST)
        notes_index(owner: user.id, query: NoteQuery::new(None, None), form: form)
    })
}
