//! Browser runtime + assets demo: signal, $(...), @event, :bind,
//! #[procedure], #[shard], asset!, AssetBundle.
//!
//! The runtime's <script> is itself an asset, so the bundle MUST exist:
//!   topcoat asset bundle --bin runtime_demo     (writes target/debug/assets/)
//!   cargo run --bin runtime_demo
//! or simply:  topcoat dev --bin runtime_demo   (builds + bundles + reloads)

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page},
    runtime::{Event, RouterBuilderRuntimeExt, procedure, shard, signal},
    view::{View, view},
};

const STYLE: Asset = asset!("./style.css");

#[tokio::main]
async fn main() {
    let router = module_router!()
        .discover() // registers #[procedure] and #[shard] endpoints
        .assets(AssetBundle::load().expect("run `topcoat asset bundle --bin runtime_demo` first"))
        .runtime() // register app layers BEFORE .runtime()
        .build();
    topcoat::start(router).await.unwrap();
}

#[procedure]
async fn double(value: usize) -> Result<usize> {
    // Callable by anyone over HTTP: validate + authorize here.
    Ok(value.saturating_mul(2))
}

#[shard]
async fn greeting(name: String) -> Result<impl View> {
    // Re-rendered on the server whenever the `name` expression changes.
    let shown = if name.is_empty() {
        "stranger".to_owned()
    } else {
        name
    };
    Ok(view! {
        <p>
            "Server says hi to "
            (shown)
        </p>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let count = signal(cx, || 0usize);
    let open = signal(cx, || false);
    let name = signal(cx, String::new);

    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <link rel="stylesheet" href=(STYLE)>
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                <button @click=$(|_e| count.set(count.get() + 1))>"+1"</button>
                <button
                    @click=$(async |_e| {
                        let doubled = double(count.get()).await;
                        count.set(doubled);
                    })
                >
                    "x2 (procedure)"
                </button>
                <p class="count">
                    "Count: "
                    $(count.get())
                </p>

                <button @click=$(|_e| open.set(!open.get()))>"Toggle"</button>
                <p :hidden=$(!open.get())>"Now you see me"</p>

                <input
                    :value=$(name.get())
                    @input=$(|e: Event| name.set(e.target.value))
                >
                greeting(name: $(name.get()))
            </body>
        </html>
    })
}
