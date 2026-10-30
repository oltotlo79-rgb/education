//! 第6巻 UNIT 04: URL の部品（パスの {id}・クエリの ?q=&page=）と、要求の中身を Cx から読む。
//! 実行: cargo run --bin u04_params

use topcoat::{
    Result,
    context::Cx,
    router::{
        module_router, page, query_params,
        request::{headers, method, uri},
    },
    view::{View, view},
};

/// 要求の中身を、1行の文字にする（&Cx があれば、どこからでも読める）
fn request_summary(cx: &Cx) -> String {
    let user_agent = headers(cx)
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("（なし）");
    format!("{} {} / User-Agent: {user_agent}", method(cx), uri(cx))
}

/// GET /whoami: 要求の中身を見せる
mod whoami {
    use super::*;

    #[page]
    async fn whoami(cx: &Cx) -> Result<impl View> {
        Ok(view! { <p>(request_summary(cx))</p> })
    }
}

mod notes {
    use super::*;

    /// URL のクエリ（?q=...&page=...）。読めなければ 400
    #[query_params(error = bad_request)]
    struct NotesQuery {
        q: Option<String>,
        page: Option<u32>,
    }

    /// GET /notes
    #[page]
    async fn list(cx: &Cx) -> Result<impl View> {
        let query = query_params::<NotesQuery>(cx)?;
        let keyword = query.q.clone().unwrap_or_else(|| "（なし）".to_string());
        let page = query.page.unwrap_or(1);
        Ok(view! {
            <p>"検索: " (keyword) " / ページ: " (page)</p>
        })
    }

    mod id {
        use topcoat::{
            Result,
            context::Cx,
            router::{module_param, page, path_param},
            view::{View, view},
        };

        // このモジュールの URL の区切りを {id} にし、u64 として読む。読めなければ 404
        module_param!(id: u64, error = not_found);

        /// GET /notes/{id}
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id = path_param::<Id>(cx)?;
            Ok(view! { <h1>"ノート " (id) " 番"</h1> })
        }
    }
}

mod users {
    mod name {
        use topcoat::{
            Result,
            context::Cx,
            router::{module_param, page, path_param},
            view::{View, view},
        };

        // 型を書かないと、区切りの文字（%XX を戻したもの）が &str で読める
        module_param!(name);

        /// GET /users/{name}
        #[page]
        async fn profile(cx: &Cx) -> Result<impl View> {
            let name: &str = path_param::<Name>(cx);
            Ok(view! { <h1>(name) " さんのページ"</h1> })
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
