//! Probe: topcoat 0.10.0 + libsql 0.10.0 (local file DB) + argon2 0.6.0 build and run
//! together natively on Windows ARM64 (MSVC).
//!
//! Run:  cargo run            (serves http://127.0.0.1:3000)
//!       curl http://127.0.0.1:3000/

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{RouterBuilderDiscoverExt, module_router, page},
    view::{View, view},
};

/// Shared DB handle stored in app context. `libsql::Connection` is cheap to clone.
struct Db {
    conn: libsql::Connection,
}

fn db(cx: &Cx) -> &libsql::Connection {
    &app_context::<Db>(cx).conn
}

#[tokio::main]
async fn main() {
    // --- libsql: local file database -------------------------------------
    let database = libsql::Builder::new_local("probe.db").build().await.unwrap();
    let conn = database.connect().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )",
        (),
    )
    .await
    .unwrap();

    // --- argon2 0.6: hash + verify ---------------------------------------
    let phc = Argon2::default()
        .hash_password(b"correct horse")
        .unwrap()
        .to_string();
    let parsed = PasswordHash::new(&phc).unwrap();
    assert!(Argon2::default().verify_password(b"correct horse", &parsed).is_ok());
    assert!(Argon2::default().verify_password(b"wrong", &parsed).is_err());
    println!("argon2 ok: {phc}");

    conn.execute(
        "INSERT OR IGNORE INTO users (email, password_hash) VALUES (?1, ?2)",
        libsql::params!["alice@example.com", phc.as_str()],
    )
    .await
    .unwrap();

    topcoat::start(
        module_router!()
            .discover()
            .app_context(Db { conn })
            .build(),
    )
    .await
    .unwrap();
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let mut rows = db(cx)
        .query("SELECT id, email FROM users ORDER BY id", ())
        .await?;
    let mut users: Vec<(i64, String)> = Vec::new();
    while let Some(row) = rows.next().await? {
        users.push((row.get::<i64>(0)?, row.get::<String>(1)?));
    }

    Ok(view! {
        <!DOCTYPE html>
        <html>
            <body>
                <h1>"users"</h1>
                <ul>
                    for (id, email) in users {
                        <li>(id) ": " (email)</li>
                    }
                </ul>
            </body>
        </html>
    })
}

// --- `?` conversions into topcoat::Error (VERIFIED: cargo check) ------------

/// Hash a password into a PHC string ("$argon2id$v=19$...").
#[allow(dead_code)]
fn hash_password(password: &str) -> Result<String> {
    Ok(Argon2::default().hash_password(password.as_bytes())?.to_string())
}

/// `Ok(false)` for a wrong password; `Err` only for a malformed stored hash.
#[allow(dead_code)]
fn verify_password(password: &str, phc: &str) -> Result<bool> {
    let parsed = PasswordHash::new(phc)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// libsql errors convert with `?` as well; argon2 is CPU-heavy, so run it on
/// the blocking pool from async handlers.
#[allow(dead_code)]
async fn hash_password_async(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || hash_password(&password)).await?
}
