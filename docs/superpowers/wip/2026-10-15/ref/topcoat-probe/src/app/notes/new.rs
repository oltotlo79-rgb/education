//! GET /notes/new : the "new note" form. (Static segment `new` wins over
//! the sibling `{note_id}` parameter segment.)

use topcoat::{
    Result,
    context::Cx,
    router::{href, page},
    view::{View, view},
};

use crate::app::require_user;

#[page]
pub(crate) async fn new_note(cx: &Cx) -> Result<impl View> {
    require_user(cx).await?;
    Ok(view! {
        <h1>"New note"</h1>
        <form method="post" action=(href!(super::create))>
            <label for="title">"Title"</label>
            <input id="title" name="title" required="">
            <label for="body">"Body"</label>
            <textarea id="body" name="body"></textarea>
            <button type="submit">"Create"</button>
        </form>
    })
}
