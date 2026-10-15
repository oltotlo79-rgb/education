//! /notes/{note_id}/edit : GET shows the form, POST validates and either
//! re-renders with 422 or redirects (303) back to the note.
//! Demonstrates `#[page(POST)]` + `Err(see_other(..).into())` from a page.

use topcoat::{
    Result,
    context::Cx,
    router::{
        StatusCode,
        content::Form,
        error::{RouterErrorExt, see_other},
        href, page, path_param,
    },
    view::{View, view},
};

use super::NoteId;
use crate::{
    app::{db, notes::NoteForm, require_user},
    components::error_list,
};

#[page]
pub(crate) async fn edit_page(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<NoteId>(cx)?;
    let note = db(cx)
        .get_note(id)
        .filter(|n| n.owner == user.name)
        .ok_or_not_found()?;

    Ok(view! {
        <h1>"Edit"</h1>
        <form method="post" action=(href!(update, NoteId(id)))>
            <input name="title" value=(note.title)>
            <textarea name="body">(note.body)</textarea>
            <button>"Save"</button>
        </form>
    })
}

#[page(POST)]
pub(crate) async fn update(cx: &Cx, Form(form): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<NoteId>(cx)?;
    db(cx)
        .get_note(id)
        .filter(|n| n.owner == user.name)
        .ok_or_not_found()?;

    let mut errors = Vec::new();
    if form.title.trim().is_empty() {
        errors.push("Title is required".to_owned());
    }
    if !errors.is_empty() {
        return Ok(view! {
            (StatusCode::UNPROCESSABLE_ENTITY)
            <h1>"Edit"</h1>
            error_list(errors: errors)
            <form method="post" action=(href!(update, NoteId(id)))>
                <input name="title" value=(form.title)>
                <textarea name="body">(form.body)</textarea>
                <button>"Save"</button>
            </form>
        });
    }

    db(cx).update_note(id, form.title, form.body);
    // A page's Ok value is a view, so a redirect leaves through Err.
    Err(see_other(href!(super::show, NoteId(id)).resolve(cx)).into())
}
