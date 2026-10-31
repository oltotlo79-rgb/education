use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    router::{Router, Slot, layout, module_router, page},
    view::{View, view},
};

/// Builds the application's router.
pub fn router() -> Router {
    // The pages and layouts of this module and its submodules, with paths derived
    // from the module tree.
    module_router!()
        .assets(AssetBundle::load().unwrap())
        .build()
}

#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"hello-topcoat"</title>
                <link rel="stylesheet" href=(asset!("styles.css"))>
                topcoat::dev::script()
            </head>
            <body>(slot)</body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main>
            <h1>"hello-topcoat"</h1>
            <p>
                "Edit "
                <code>"src/app.rs"</code>
                " and save to see your changes."
            </p>
        </main>
    })
}
