use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{module_router, page},
    tailwind,
    view::{View, view},
};

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("run `topcoat asset bundle` first"))
        .build();
    topcoat::start(router).await.unwrap();
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head><link rel="stylesheet" href=(tailwind::stylesheet!())></head>
            <body><h1 class="text-3xl font-bold text-blue-600">"Tailwind"</h1></body>
        </html>
    })
}
