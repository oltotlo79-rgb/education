//! フォームの構造体に #[derive(Deserialize)] を付け忘れた
use topcoat::{
    Result,
    router::{content::Form, module_router, page},
    view::{View, view},
};

struct MemoForm {
    title: String,
}

#[page(POST)]
async fn create(Form(input): Form<MemoForm>) -> Result<impl View> {
    Ok(view! { <p>(input.title)</p> })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
