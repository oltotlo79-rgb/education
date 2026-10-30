//! 問2の解答: 式で書いた属性と、文字で書いた属性の違いを確かめる。
//! 実行: cargo run --bin u05_q2

use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    let maybe_title: Option<&str> = None;
    let is_disabled = false;
    let expanded = false;
    Ok(view! {
        <button title=(maybe_title) disabled=(is_disabled)>"A"</button>
        <button disabled="false">"B"</button>
        <button aria-expanded=(if expanded { "true" } else { "false" })>"C"</button>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
