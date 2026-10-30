//! 閉じタグ </p> を書き忘れた
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <section>
            <p>"1つ目の段落"
            <p>"2つ目の段落"</p>
        </section>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
