//! view! に入れた String を、あとでもう一度使おうとした
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    let title = String::from("買い物メモ");
    let header = view! { <h1>(title)</h1> };
    Ok(view! {
        (header)
        <p>"タイトルは " (title.len()) " バイトです"</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
