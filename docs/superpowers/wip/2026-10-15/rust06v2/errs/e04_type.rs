//! path_param の結果を、そのまま u64 として受け取ろうとした
use topcoat::{
    Result,
    context::Cx,
    router::{module_param, module_router, page, path_param},
    view::{View, view},
};

mod notes {
    mod id {
        use super::super::*;

        module_param!(id: u64); // error = ... を書いていない

        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id: u64 = path_param::<Id>(cx);
            Ok(view! { <h1>"ノート " (id) " 番"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
