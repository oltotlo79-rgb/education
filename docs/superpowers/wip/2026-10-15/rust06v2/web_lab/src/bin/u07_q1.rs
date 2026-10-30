//! 問1の解答: u07_memo から #[memoize(as_ref)] を外したもの。
//! 実行: cargo run --bin u07_q1

use std::time::Duration;

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, layout, module_router, page},
    view::{View, component, view},
};

/// DB から読んだ、と考えるユーザー
struct User {
    name: String,
}

/// ユーザーを「DB から」読む（ここでは 50 ミリ秒待ってから作る）。
/// #[memoize] を外したので、呼ぶたびに読む。
async fn load_user(_cx: &Cx, id: u32) -> Option<User> {
    println!("  DB を読みました（id = {id}）");
    tokio::time::sleep(Duration::from_millis(50)).await;
    (id == 1).then(|| User {
        name: "alice".to_string(),
    })
}

/// 見出しに名前を出す部品
#[component]
async fn user_badge(cx: &Cx) -> Result<impl View> {
    let name = load_user(cx, 1).await.map(|u| u.name.clone());
    Ok(view! { <span class="badge">(name)</span> })
}

/// レイアウトでも、同じユーザーを読む
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let signed_in = load_user(cx, 1).await.is_some();
    Ok(view! {
        <header>
            if signed_in {
                "ログイン中: " user_badge()
            }
        </header>
        (slot)
    })
}

/// GET /: ページでも、同じユーザーと、別のユーザー（id = 2）を読む
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let alice = load_user(cx, 1).await.map(|u| u.name.clone());
    let other = load_user(cx, 2).await.is_some();
    Ok(view! {
        <p>"ページ: " (alice) " / id 2 は " (if other { "いる" } else { "いない" })</p>
        user_badge()
        user_badge()
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    println!("http://127.0.0.1:3000 で待っています");
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
