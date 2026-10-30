//! 第6巻 UNIT 08 の例8-2: topcoat dev で動かすと、保存するたびに作り直し、ブラウザも読み直す。
//! 実行: topcoat dev --bin u08_dev

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <link rel="stylesheet" href=(SITE_CSS)>
                // topcoat dev で動かしているときだけ、自動で読み直すためのスクリプトが入る
                topcoat::dev::script()
            </head>
            <body>
                <h1>"開発サーバーの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
