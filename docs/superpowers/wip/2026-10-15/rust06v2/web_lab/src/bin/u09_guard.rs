//! 第6巻 UNIT 09 の例9-2: ページの確かめは、procedure を守らない。procedure の中でも確かめる。
//! 実行: topcoat asset bundle --bin u09_guard のあとで cargo run --bin u09_guard

use std::sync::Mutex;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::{Cx, app_context},
    router::{RouterBuilderDiscoverExt, error::forbidden, module_router, page, request::headers},
    runtime::{RouterBuilderRuntimeExt, procedure},
    view::{View, view},
};

/// 保存したメモ
struct Memos(Mutex<Vec<String>>);

/// 管理者か（この例では、Cookie: admin=yes が付いていれば管理者とみなす。本物はセッションで確かめる）
fn is_admin(cx: &Cx) -> bool {
    headers(cx)
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|cookie| cookie.split("; ").any(|c| c == "admin=yes"))
}

/// してはいけない例: 中で確かめずに、メモを全部消す
#[procedure("/api/clear")]
async fn clear_unsafe(cx: &Cx) -> Result<usize> {
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

/// 正しい例: procedure の中でも、管理者かを確かめる
#[procedure("/api/clear-safe")]
async fn clear_safe(cx: &Cx) -> Result<usize> {
    if !is_admin(cx) {
        return Err(forbidden().into());
    }
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

mod admin {
    use super::*;

    /// GET /admin: 管理者だけが開けるページ。ボタンで procedure を呼ぶ
    #[page]
    async fn admin_page(cx: &Cx) -> Result<impl View> {
        if !is_admin(cx) {
            return Err(forbidden().into());
        }
        let count = app_context::<Memos>(cx).0.lock().unwrap().len();
        Ok(view! {
            <head>topcoat::runtime::script()</head>
            <p>"メモは " (count) " 件"</p>
            <button @click=$(async |_e| {
                clear_safe().await;
            })>
                "全部消す"
            </button>
        })
    }
}

#[tokio::main]
async fn main() {
    let memos = vec!["牛乳".to_string(), "本".to_string(), "宿題".to_string()];
    let router = module_router!()
        .discover()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .app_context(Memos(Mutex::new(memos)))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
