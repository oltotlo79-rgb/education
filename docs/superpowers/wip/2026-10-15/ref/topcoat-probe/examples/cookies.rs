//! Cookie jar, `cookie!` macro, private (encrypted) cookies, typed cookie store.
//! Run: cargo run --example cookies
//!   curl -i -X POST localhost:3000/theme            -> Set-Cookie: theme=dark
//!   curl -i -X POST localhost:3000/flash            -> Set-Cookie: flash=<encrypted>
//!   curl -i -X POST localhost:3000/cart             -> Set-Cookie: cart=<encrypted JSON>

use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    context::Cx,
    cookie::{
        Cookies, Key, RouterBuilderCookieExt, SameSite, cookie, cookie_store, cookies,
        private_cookies,
    },
    router::{module_router, route},
};

#[tokio::main]
async fn main() {
    topcoat::start(
        module_router!()
            .cookies() // required for cookies(cx)
            // Key for signed/private cookies. Key::generate() = new key per start;
            // load a persisted key (e.g. Key::from(&bytes)) to survive restarts.
            .app_context(Key::generate())
            .build(),
    )
    .await
    .unwrap();
}

mod theme {
    use super::*;

    #[route(POST)]
    async fn toggle(cx: &Cx) -> Result<String> {
        let jar = cookies(cx);
        let next = match jar.get("theme") {
            Some(c) if c.value() == "dark" => "light",
            _ => "dark",
        };
        jar.add(cookie! {
            "theme" = next;
            Path = "/";
            HttpOnly;
            SameSite = Lax
        });
        Ok(next.to_owned())
    }
}

mod flash {
    use super::*;

    #[route(POST)]
    async fn set(cx: &Cx) -> Result<&'static str> {
        private_cookies(cx).add(cookie!("flash" = "saved!"; Path = "/"));
        Ok("ok")
    }

    #[route(GET)]
    async fn read(cx: &Cx) -> Result<String> {
        let jar = private_cookies(cx).default_same_site(SameSite::Lax);
        Ok(jar
            .get("flash")
            .map(|c| c.value().to_owned())
            .unwrap_or_default())
    }
}

mod cart {
    use super::*;

    #[derive(Default, Serialize, Deserialize)]
    struct Cart {
        items: Vec<String>,
    }

    #[route(POST)]
    async fn add(cx: &Cx) -> Result<String> {
        let cart = cookie_store::<Cart, _>(private_cookies(cx), "cart")
            .parse_or_default()
            .update(|c| c.items.push("widget".to_owned()))
            .commit()?;
        Ok(format!("{} items", cart.items.len()))
    }
}
