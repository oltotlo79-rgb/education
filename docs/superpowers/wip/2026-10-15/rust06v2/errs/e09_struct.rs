//! 自分で作った構造体を、そのまま signal に入れた
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{module_router, page},
    runtime::{RouterBuilderRuntimeExt, signal},
    view::{View, view},
};

#[derive(Clone)]
struct Cart {
    items: usize,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let cart = signal(cx, || Cart { items: 0 });
    Ok(view! {
        <head>topcoat::runtime::script()</head>
        <p>"かご: " $(cart.get().items)</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router).await.expect("起動できません");
}
