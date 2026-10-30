//! 第13巻 UNIT 08 エラー3: view! の for に、Clone できない値の並びを、そのまま渡した。
//! 実行: cargo run -q --example e08_key
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{View, component, view},
};

struct Item {
    title: String,
}

#[component]
async fn item_list(items: Vec<Item>) -> Result<impl View> {
    Ok(view! {
        <ul>
            for item in items {
                <li>(item.title)</li>
            }
        </ul>
    })
}

fn main() {}
