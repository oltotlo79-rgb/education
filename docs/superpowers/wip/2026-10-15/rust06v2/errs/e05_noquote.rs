//! view! の中の文字を、"..." で囲み忘れた
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>こんにちは</h1>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
