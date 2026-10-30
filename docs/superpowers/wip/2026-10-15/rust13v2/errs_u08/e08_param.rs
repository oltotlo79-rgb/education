//! 第13巻 UNIT 08 エラー1: {id} のあるページへのリンクを、ID を渡さずに作った。
//! 実行: cargo run -q --example e08_param
#![allow(dead_code)]

use rustnote::routes::notes::id;
use topcoat::{
    Result,
    router::href,
    view::{View, component, view},
};

#[component]
async fn note_link() -> Result<impl View> {
    Ok(view! {
        <a href=(href!(id::show))>"ノートを開く"</a>
    })
}

fn main() {}
