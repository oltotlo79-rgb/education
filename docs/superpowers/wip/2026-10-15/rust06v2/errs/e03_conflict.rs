//! 同じモジュールに、GET のページを2つ書いてしまった
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    #[page]
    async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }

    // 書き足したつもりが、同じモジュール（/about）に入れてしまった
    #[page]
    async fn contact_page() -> Result<impl View> {
        Ok(view! { <h1>"お問い合わせ"</h1> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
