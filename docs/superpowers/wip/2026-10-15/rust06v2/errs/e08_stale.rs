//! asset! を足したのに、束を作り直さずに起動した
use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

const SITE_CSS: Asset = asset!("assets/site.css");
// あとから足した CSS
const PRINT_CSS: Asset = asset!("assets/print.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <link rel="stylesheet" href=(SITE_CSS)>
        <link rel="stylesheet" href=(PRINT_CSS) media="print">
        <h1>"アセットの練習"</h1>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router).await.expect("起動できません");
}
