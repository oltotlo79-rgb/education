//! /notes/{note_id} : show (GET) and delete (POST ./delete).
//!
//! `module_param!` (new in 0.10) declares the `NoteId` type AND makes this
//! module's URL segment `{note_id}` (the file name does not matter).

pub(crate) mod edit;

use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{RouterErrorExt, SeeOther, see_other},
        href, module_param, page, path_param, route,
    },
    view::{View, view},
};

use crate::app::{db, require_user};

// Non-numeric segment (e.g. /notes/abc) -> 404 NotFoundError.
module_param!(pub(crate) note_id: u64, error = not_found);

#[page]
pub(crate) async fn show(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id: u64 = *path_param::<NoteId>(cx)?;
    let note = db(cx)
        .get_note(id)
        .filter(|n| n.owner == user.name)
        .ok_or_not_found()?;

    Ok(view! {
        <article>
            <h1>(note.title)</h1>
            // Text is HTML-escaped automatically (e.g. "<script>" becomes "&lt;script>").
            <p>(note.body)</p>
        </article>
        <a href=(href!(edit::edit_page, NoteId(id)))>"Edit"</a>
        <form method="post" action=(href!(delete, NoteId(id)))>
            <button>"Delete"</button>
        </form>
    })
}

/// POST /notes/{note_id}/delete  (relative path "./delete" extends the module path)
#[route(POST "./delete")]
pub(crate) async fn delete(cx: &Cx) -> Result<SeeOther> {
    let user = require_user(cx).await?;
    let id = *path_param::<NoteId>(cx)?;
    let note = db(cx)
        .get_note(id)
        .filter(|n| n.owner == user.name)
        .ok_or_not_found()?;
    db(cx).delete_note(note.id);
    Ok(see_other(href!(super::index).resolve(cx)))
}
