//! Minimal app with EXPLICIT paths, collected by `Router::builder().discover()`.
//! Run:  cargo run --example hello          -> http://127.0.0.1:3000
//!       PORT=8080 HOST=0.0.0.0 cargo run --example hello   (Fly.io style)

use topcoat::{
    Result,
    context::Cx,
    router::{
        Router, RouterBuilderDiscoverExt, page, path_param,
        request::{headers, method, uri},
        route,
    },
    view::{View, component, view},
};

#[tokio::main]
async fn main() {
    // `.discover()` comes from the RouterBuilderDiscoverExt trait (must be imported).
    let router = Router::builder().discover().build();
    topcoat::start(router).await.unwrap();
}

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head><title>"Hello"</title></head>
            <body>hello(name: "World")</body>
        </html>
    })
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    })
}

// Explicit-path parameter: write `{user_id}` in the path yourself.
path_param!(user_id: u32, error = bad_request);

#[page("/users/{user_id}")]
async fn user(cx: &Cx) -> Result<impl View> {
    let id: &u32 = path_param::<UserId>(cx)?;
    Ok(view! {
        <p>
            "user #"
            (id)
        </p>
    })
}

// Plain-text route that echoes request info.
#[route(GET "/whoami")]
async fn whoami(cx: &Cx) -> Result<String> {
    let ua = headers(cx)
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    Ok(format!("{} {} ua={ua}", method(cx), uri(cx).path()))
}
