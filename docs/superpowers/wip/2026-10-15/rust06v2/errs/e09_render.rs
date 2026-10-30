//! procedure の呼び出しを、ボタンの中ではなく、表示の式に書いてしまった
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{RouterBuilderDiscoverExt, module_router, page},
    runtime::{RouterBuilderRuntimeExt, procedure},
    view::{View, view},
};

#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    Ok(value * 2)
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <head>topcoat::runtime::script()</head>
        <p>"2倍: " $(double(21))</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .discover()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router).await.expect("起動できません");
}
