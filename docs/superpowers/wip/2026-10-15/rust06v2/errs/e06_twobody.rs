//! 1つの関数で、フォームと JSON の両方を受け取ろうとした
use serde::Deserialize;
use topcoat::{
    Result,
    router::{
        content::{Form, Json},
        module_router, page,
    },
    view::{View, view},
};

#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

#[page(POST)]
async fn create(Form(form): Form<MemoForm>, Json(json): Json<MemoForm>) -> Result<impl View> {
    Ok(view! { <p>(form.title) (json.title)</p> })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
