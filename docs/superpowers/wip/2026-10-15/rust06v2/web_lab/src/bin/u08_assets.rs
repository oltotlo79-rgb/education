//! 第6巻 UNIT 08: CSS を asset! で宣言し、topcoat asset bundle で束にして配る。
//! 実行: topcoat asset bundle --bin u08_assets のあとで cargo run --bin u08_assets

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

/// クレートの assets/site.css（Cargo.toml のあるフォルダからの場所）
const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 08"</title>
                <link rel="stylesheet" href=(SITE_CSS)>
            </head>
            <body>
                <h1>"アセットの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // 実行ファイルの隣の assets/ から、束を読み込む
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
