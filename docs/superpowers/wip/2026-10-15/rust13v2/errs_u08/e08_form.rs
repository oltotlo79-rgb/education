//! 第13巻 UNIT 08 エラー3: フォームを読む型に、#[derive(Deserialize)] を付け忘れた。
//! 実行: cargo run -q --example e08_form
#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page},
    view::{View, view},
};

pub struct NoteForm {
    title: String,
    body: String,
}

#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let _ = cx;
    Ok(view! {
        <p>(input.title)</p>
    })
}

fn main() {}
