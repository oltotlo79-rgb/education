//! 第6巻 UNIT 09: ブラウザの中で動く部分（signal・イベント・procedure）。
//! 実行: topcoat asset bundle --bin u09_runtime のあとで cargo run --bin u09_runtime

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page},
    runtime::{RouterBuilderRuntimeExt, procedure, signal},
    view::{View, view},
};

/// サーバーで動く関数。ブラウザから POST /api/double で呼ばれる
#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    Ok(value * 2)
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // ブラウザが持つ状態。最初の値はサーバーが決めて送る
    let count = signal(cx, || 1usize);
    let open = signal(cx, || false);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 09"</title>
                // ブラウザで動く部分（runtime）のスクリプト
                topcoat::runtime::script()
            </head>
            <body>
                <p>"数: " $(count.get())</p>
                // ブラウザの中だけで 1 足す（サーバーには送らない）
                <button @click=$(|_e| count.increment())>"+1"</button>
                // サーバーの double を呼んで、結果を入れる
                <button @click=$(async |_e| {
                    let doubled = double(count.get()).await;
                    count.set(doubled);
                })>
                    "2倍（サーバーで計算）"
                </button>
                <button @click=$(|_e| open.set(!open.get()))>"説明"</button>
                <p :hidden=$(!open.get())>"この文は、ブラウザの中だけで出したり消したりします"</p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // procedure（double）を集めて登録する
        .discover()
        // runtime のスクリプトは、アセットの束から配る
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
