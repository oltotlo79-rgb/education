//! Route tree root. `module_router!()` is called in this module, so this
//! module maps to "/" and each child module adds one kebab-case URL segment.
//!
//!   app.rs                    -> GET /             (+ root layout, layer, not_found!)
//!   app/login.rs              -> GET /login, POST /login
//!   app/logout.rs             -> POST /logout
//!   app/notes.rs              -> GET /notes (query ?q=), POST /notes
//!   app/notes/new.rs          -> GET /notes/new
//!   app/notes/note_id.rs      -> GET /notes/{note_id}, POST /notes/{note_id}/delete
//!   app/notes/note_id/edit.rs -> GET /notes/{note_id}/edit, POST /notes/{note_id}/edit
//!   app/api/health.rs         -> GET /api/health (JSON)
//!   app/api/notes.rs          -> GET /api/notes, POST /api/notes (JSON)

pub(crate) mod api;
pub(crate) mod login;
pub(crate) mod logout;
pub(crate) mod notes;

use std::{
    collections::{BTreeMap, HashMap},
    sync::{Mutex, PoisonError},
    time::SystemTime,
};

use topcoat::{
    Result,
    context::{Cx, app_context, memoize},
    cookie::RouterBuilderCookieExt,
    router::{
        Body, Next, Router, Slot, StatusCode,
        error::{NotFoundError, RouterErrorExt},
        href, layer, layout, module_router, not_found, page,
        request::{method, uri},
        response::Response,
    },
    session::{self, RouterBuilderSessionExt, SessionConfig, TokenHash},
    view::{View, error_boundary, view},
};

use crate::components::card;

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router() -> Router {
    module_router!()
        .cookies()
        .sessions(SessionConfig::default())
        .app_context(Db::default())
        .app_context(Config {
            site_name: "RustNote probe".to_owned(),
        })
        .build()
}

// Every URL nothing else matches becomes a NotFoundError that flows through
// the layouts (so the root layout's error_boundary can render a branded 404).
not_found!();

// ---------------------------------------------------------------------------
// Layer (wraps request handling under "/")
// ---------------------------------------------------------------------------

#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    let status = match &result {
        Ok(response) => response.status().to_string(),
        Err(_) => "error".to_owned(),
    };
    eprintln!(
        "{} {} -> {} ({:?})",
        method(cx),
        uri(cx),
        status,
        started.elapsed()
    );
    result
}

// ---------------------------------------------------------------------------
// Root layout + home page
// ---------------------------------------------------------------------------

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let user = current_user(cx).await;
    let site_name = config(cx).site_name.as_str();

    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>(site_name)</title>
                topcoat::dev::script()
            </head>
            <body>
                <nav>
                    <a href=(href!(home))>"Home"</a>
                    " "
                    <a href=(href!(notes::index))>"Notes"</a>
                    " "
                    if let Some(user) = user {
                        <span class="user">(user.name.as_str())</span>
                        <form method="post" action=(href!(logout::logout))>
                            <button>"Log out"</button>
                        </form>
                    } else {
                        <a href=(href!(login::login_page))>"Log in"</a>
                    }
                </nav>
                <main>
                    error_boundary(
                        fallback: |error| {
                            if error.downcast_ref::<NotFoundError>().is_none() {
                                // Not a 404: rethrow so the router turns it into its response.
                                return Err(error);
                            }
                            Ok(
                                view! {
                                    (StatusCode::NOT_FOUND)
                                    <h1>"404: page not found"</h1>
                                },
                            )
                        },
                        (slot)
                    )
                </main>
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let user = current_user(cx).await;
    Ok(view! {
        card(
            title: "Welcome",
            highlighted: true,
            <p>
                match user {
                    Some(u) => {
                        "Signed in as "
                        (u.name.as_str())
                    }
                    None => "Not signed in",
                }
            </p>
        )
    })
}

// ---------------------------------------------------------------------------
// App context: config + in-memory "database"
// ---------------------------------------------------------------------------

pub struct Config {
    pub site_name: String,
}

pub fn config(cx: &Cx) -> &Config {
    app_context::<Config>(cx)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Note {
    pub id: u64,
    pub owner: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

#[derive(Default)]
pub struct Db {
    inner: Mutex<DbInner>,
}

#[derive(Default)]
struct DbInner {
    next_id: u64,
    notes: BTreeMap<u64, Note>,
    sessions: HashMap<TokenHash, (User, SystemTime)>,
}

impl Db {
    fn lock(&self) -> std::sync::MutexGuard<'_, DbInner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn create_note(&self, owner: &str, title: String, body: String) -> u64 {
        let mut db = self.lock();
        db.next_id += 1;
        let id = db.next_id;
        db.notes.insert(
            id,
            Note {
                id,
                owner: owner.to_owned(),
                title,
                body,
            },
        );
        id
    }

    pub fn list_notes(&self, owner: &str, q: Option<&str>) -> Vec<Note> {
        self.lock()
            .notes
            .values()
            .filter(|n| n.owner == owner)
            .filter(|n| q.is_none_or(|q| n.title.contains(q)))
            .cloned()
            .collect()
    }

    pub fn get_note(&self, id: u64) -> Option<Note> {
        self.lock().notes.get(&id).cloned()
    }

    pub fn update_note(&self, id: u64, title: String, body: String) {
        if let Some(n) = self.lock().notes.get_mut(&id) {
            n.title = title;
            n.body = body;
        }
    }

    pub fn delete_note(&self, id: u64) {
        self.lock().notes.remove(&id);
    }

    pub fn create_session(&self, session: session::Session, user: User) {
        self.lock()
            .sessions
            .insert(session.token_hash, (user, session.expires_at));
    }

    pub fn session_user(&self, hash: &TokenHash) -> Option<User> {
        self.lock()
            .sessions
            .get(hash)
            .filter(|(_, expires_at)| *expires_at > SystemTime::now())
            .map(|(user, _)| user.clone())
    }

    pub fn delete_session(&self, hash: &TokenHash) {
        self.lock().sessions.remove(hash);
    }
}

pub fn db(cx: &Cx) -> &Db {
    app_context::<Db>(cx)
}

// ---------------------------------------------------------------------------
// Auth helpers ("functions, not middlewares")
// ---------------------------------------------------------------------------

/// Current user for this request. `#[memoize(as_ref)]` caches the owned
/// `Option<User>` per request and hands callers an `Option<&User>`.
#[memoize(as_ref)]
pub async fn current_user(cx: &Cx) -> Option<User> {
    let hash = session::token_hash(cx).await.ok().flatten()?;
    db(cx).session_user(&hash)
}

/// The current user, or a redirect (307) to the login page.
pub async fn require_user(cx: &Cx) -> Result<&User> {
    Ok(current_user(cx)
        .await
        .ok_or_redirect(href!(login::login_page).resolve(cx))?)
}
