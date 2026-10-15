//! /notes : list (GET, with ?q= query) and create (POST form -> 303).

pub(crate) mod new;
pub(crate) mod note_id;

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{
        content::Form,
        error::{SeeOther, bad_request, see_other},
        href, page, query_params, route,
    },
    view::{View, view},
};

use crate::app::{db, require_user};

#[query_params(error = bad_request)]
pub(crate) struct NotesQuery {
    q: Option<String>,
}

#[page]
pub(crate) async fn index(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let query = query_params::<NotesQuery>(cx)?;
    let notes = db(cx).list_notes(&user.name, query.q.as_deref());

    Ok(view! {
        <h1>"Notes"</h1>
        <form method="get" action=(href!(index))>
            <input name="q" value=(query.q.as_deref())>
            <button>"Search"</button>
        </form>
        <p><a href=(href!(new::new_note))>"New note"</a></p>
        if notes.is_empty() {
            <p>"No notes yet."</p>
        } else {
            <ul>
                #[key(note.id)]
                for note in notes {
                    <li>
                        <a href=(href!(note_id::show, note_id::NoteId(note.id)))>
                            (note.title)
                        </a>
                    </li>
                }
            </ul>
        }
    })
}

/// Fields of the HTML form (application/x-www-form-urlencoded).
#[derive(Deserialize)]
pub(crate) struct NoteForm {
    pub title: String,
    pub body: String,
}

/// POST /notes : create, then Post/Redirect/Get with 303.
#[route(POST)]
pub(crate) async fn create(cx: &Cx, Form(form): Form<NoteForm>) -> Result<SeeOther> {
    let user = require_user(cx).await?;
    if form.title.trim().is_empty() {
        return Err(bad_request("title is required").into());
    }
    let id = db(cx).create_note(&user.name, form.title, form.body);
    Ok(see_other(
        href!(note_id::show, note_id::NoteId(id)).resolve(cx),
    ))
}
