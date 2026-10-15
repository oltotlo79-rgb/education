//! 0.10 BREAKING CHANGE demo: `path_param!` no longer changes the module's URL
//! segment under `module_router!`. Use `module_param!` for that.
//! Run: cargo run --example path_param_gotcha, then
//!   curl -i localhost:3000/items/7        -> 200 "item 7"  (module_param!)
//!   curl -i localhost:3000/legacy/id      -> 200 "literal /legacy/id"  (path_param! keeps segment "id")
//!   curl -i localhost:3000/legacy/7       -> 404
//!   curl -i localhost:3000/posts/5        -> 200 "post 5"  (path_param! + relative "./{post_id}")

use topcoat::{
    Result,
    router::module_router,
    view::{View, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(module_router!().build()).await.unwrap();
}

#[topcoat::router::page]
async fn home() -> Result<impl View> {
    Ok(view! { "home" })
}

mod items {
    pub mod item_id {
        use topcoat::{
            Result,
            context::Cx,
            router::{module_param, page, path_param},
            view::{View, view},
        };

        // Sets this module's segment to {item_id}: serves /items/{item_id}
        module_param!(item_id: u32, error = not_found);

        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id = path_param::<ItemId>(cx)?;
            Ok(view! {
                "item "
                (id)
            })
        }
    }
}

mod legacy {
    pub mod id {
        use topcoat::{
            Result,
            router::{page, path_param},
            view::{View, view},
        };

        // 0.6-0.9 code: this USED TO turn the module into {legacy_id}.
        // In 0.10 it only declares the type; the module keeps its static
        // segment "id", so the page below is served at /legacy/id.
        path_param!(legacy_id: u32);

        #[page]
        async fn show() -> Result<impl View> {
            Ok(view! { "literal /legacy/id" })
        }
    }
}

mod posts {
    use topcoat::{
        Result,
        context::Cx,
        router::{page, path_param},
        view::{View, view},
    };

    // path_param! + a relative path adds the segment explicitly.
    path_param!(post_id: u64, error = bad_request);

    #[page("./{post_id}")]
    async fn show(cx: &Cx) -> Result<impl View> {
        let id = path_param::<PostId>(cx)?;
        Ok(view! {
            "post "
            (id)
        })
    }
}
