//! href! に、ページではない、ふつうの関数の名前を渡してしまった
use topcoat::{
    Result,
    router::{href, module_router, page},
    view::{View, view},
};

/// ページではない、ただの関数（#[page] を付け忘れた）
async fn about_page() -> Result<String> {
    Ok("この場所について".to_string())
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>"トップ"</h1>
        <a href=(href!(about_page))>"この場所について"</a>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
