//! 第13巻 UNIT 07 エラー1: view! の中の文字を、" " で囲まずに書いた。
//! 実行: cargo run -q --example e07_text
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
async fn hero() -> Result<impl View> {
    Ok(view! {
        <h1>RustNote</h1>
    })
}

fn main() {}
