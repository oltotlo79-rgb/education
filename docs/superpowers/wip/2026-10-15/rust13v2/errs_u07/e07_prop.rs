//! 第13巻 UNIT 07 エラー2: 部品（コンポーネント）を、引数を渡さずに使った。
//! 実行: cargo run -q --example e07_prop
#![allow(dead_code)]

use rustnote::views::flash_banner;
use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
async fn page_top() -> Result<impl View> {
    Ok(view! {
        flash_banner()
    })
}

fn main() {}
