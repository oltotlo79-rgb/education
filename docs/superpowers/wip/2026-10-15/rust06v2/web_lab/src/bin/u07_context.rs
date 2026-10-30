//! 第6巻 UNIT 07: アプリ全体で共有する値（設定・数え上げ）を、app_context でどのページからも借りる。
//! 実行: cargo run --bin u07_context

use std::sync::atomic::{AtomicU64, Ordering};

use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{module_router, page},
    view::{View, view},
};

/// アプリの設定（起動のときに1回だけ作る）
struct AppConfig {
    site_name: String,
}

/// 来た要求の数（どの要求からも同時に足せるように、AtomicU64 にする）
struct Visits(AtomicU64);

/// 設定を借りる小さな関数（app_context::<AppConfig> を、毎回書かなくて済む）
fn config(cx: &Cx) -> &AppConfig {
    app_context::<AppConfig>(cx)
}

/// 数を1つ足して、足したあとの数を返す
fn count_visit(cx: &Cx) -> u64 {
    app_context::<Visits>(cx).0.fetch_add(1, Ordering::Relaxed) + 1
}

/// GET /
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let visits = count_visit(cx);
    Ok(view! {
        <h1>(config(cx).site_name.as_str())</h1>
        <p>(visits) " 回目の表示です"</p>
    })
}

mod about {
    use super::*;

    /// GET /about（別のページからも、同じ値を借りられる）
    #[page]
    async fn about(cx: &Cx) -> Result<impl View> {
        let visits = count_visit(cx);
        Ok(view! {
            <p>(config(cx).site_name.as_str()) " について（" (visits) " 回目）"</p>
        })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // 型ごとに1つずつ登録する
        .app_context(AppConfig {
            site_name: "UNIT 07 のメモ帳".to_string(),
        })
        .app_context(Visits(AtomicU64::new(0)))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
