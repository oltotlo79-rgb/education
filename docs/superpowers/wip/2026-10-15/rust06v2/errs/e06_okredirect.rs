//! 303 で移るつもりで、Ok(see_other(...)) と書いた
use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, module_router, page},
    view::{View, view},
};

#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"メモ"</h1> })
}

#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<MemoForm>) -> Result<impl View> {
    if input.title.is_empty() {
        return Ok(view! { <p>"タイトルを入力してください"</p> });
    }
    Ok(see_other(href!(home).resolve(cx)))
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
