//! GET /login shows the form; POST /login checks credentials, starts a
//! session (Set-Cookie: __Host-session=...; Secure; HttpOnly; SameSite=Lax)
//! and redirects with 303. Wrong password re-renders the form with 401.

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page},
    session,
    view::{View, component, view},
};

use crate::app::{User, db};

#[derive(Deserialize)]
pub(crate) struct LoginForm {
    name: String,
    password: String,
}

#[component]
async fn login_form(#[default] error: Option<&str>) -> Result<impl View> {
    Ok(view! {
        if let Some(error) = error {
            <p class="error">(error)</p>
        }
        <form method="post" action=(href!(login))>
            <input name="name" placeholder="Name" required="">
            <input name="password" type="password" required="">
            <button>"Log in"</button>
        </form>
    })
}

#[page]
pub(crate) async fn login_page() -> Result<impl View> {
    Ok(view! {
        <h1>"Log in"</h1>
        login_form()
    })
}

#[page(POST)]
pub(crate) async fn login(cx: &Cx, Form(form): Form<LoginForm>) -> Result<impl View> {
    // Demo only: a real app verifies an Argon2 hash from the database here.
    if form.password != "secret" {
        return Ok(view! {
            (StatusCode::UNAUTHORIZED)
            <h1>"Log in"</h1>
            login_form(error: Some("Invalid name or password"))
        });
    }

    // Always mints a fresh token (prevents session fixation).
    let session = session::start(cx).await?;
    db(cx).create_session(session, User { name: form.name });

    Err(see_other(href!(crate::app::notes::index).resolve(cx)).into())
}
