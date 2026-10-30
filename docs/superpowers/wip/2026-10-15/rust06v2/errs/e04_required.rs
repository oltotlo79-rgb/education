//! クエリの項目を Option にし忘れた（page: u32）
use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page, query_params},
    view::{View, view},
};

mod notes {
    use super::*;

    #[query_params(error = bad_request)]
    struct NotesQuery {
        page: u32, // Option<u32> にし忘れた
    }

    #[page]
    async fn list(cx: &Cx) -> Result<impl View> {
        let query = query_params::<NotesQuery>(cx)?;
        Ok(view! { <p>"ページ: " (query.page)</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
