//! 第13巻 UNIT 07 エラー3: href! に、ページではない関数を渡した。
//! 実行: cargo run -q --example e07_href
#![allow(dead_code)]

use topcoat::{
    Result,
    router::href,
    view::{View, component, view},
};

#[component]
async fn back_link() -> Result<impl View> {
    Ok(view! {
        <a href=(href!(rustnote::views::site_header))>"戻る"</a>
    })
}

fn main() {}
