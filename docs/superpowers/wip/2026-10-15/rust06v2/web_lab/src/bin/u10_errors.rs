//! 第6巻 UNIT 10: エラーを、自分の画面（状態コード付き）に変える。404・400・リダイレクト・500。
//! 実行: cargo run --bin u10_errors

use topcoat::{
    Error, Result,
    context::Cx,
    router::{
        Slot, StatusCode,
        error::{BadRequestError, NotFoundError, bad_request, redirect},
        layout, module_router, not_found, page, query_params,
    },
    view::{View, error_boundary, view},
};

// どのルートにも当たらない URL も、NotFoundError としてレイアウトまで届ける
not_found!();

/// ページで起きたエラーを、状態コード付きの自分の画面に変える。
/// 知らないエラー（リダイレクトや 500 になるもの）は、そのまま Topcoat に返す
fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> {
    let (status, message) = if error.downcast_ref::<NotFoundError>().is_some() {
        (StatusCode::NOT_FOUND, "ページが見つかりません".to_string())
    } else if let Some(bad) = error.downcast_ref::<BadRequestError>() {
        (StatusCode::BAD_REQUEST, bad.description().to_string())
    } else {
        return Err(error);
    };
    Ok(view! {
        cx =>
        (status)
        <h1>(message)</h1>
        <p><a href="/">"トップへ"</a></p>
    })
}

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <header>"UNIT 10 のサイト"</header>
        <main>
            error_boundary(fallback: |error| error_page(cx, error), (slot))
        </main>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod items {
    mod id {
        use topcoat::{
            Result,
            context::Cx,
            router::{error::RouterErrorExt, module_param, page, path_param},
            view::{View, view},
        };

        module_param!(id: u32, error = not_found);

        /// GET /items/{id}: 1〜3 番だけある
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id = *path_param::<Id>(cx)?;
            let name = ["りんご", "みかん", "ぶどう"]
                .get(id.wrapping_sub(1) as usize)
                .ok_or_not_found()?;
            Ok(view! { <h1>(id) " 番: " (*name)</h1> })
        }
    }
}

mod search {
    use super::*;

    #[query_params]
    struct SearchQuery {
        q: Option<String>,
    }

    /// GET /search?q=…: 言葉が無ければ 400
    #[page]
    async fn search(cx: &Cx) -> Result<impl View> {
        let q = query_params::<SearchQuery>(cx)
            .ok()
            .and_then(|query| query.q.clone())
            .filter(|q| !q.trim().is_empty())
            .ok_or_else(|| bad_request("検索の言葉を入れてください"))?;
        Ok(view! { <h1>"「" (q) "」の検索結果"</h1> })
    }
}

mod old {
    use super::*;

    /// GET /old: 古い URL。トップへ移す（画面は返さないので、戻り値は ()）
    #[page]
    async fn old() -> Result<()> {
        Err(redirect("/").into())
    }
}

mod boom {
    use super::*;

    /// GET /boom: ルーターのエラーではない、ふつうのエラー
    #[page]
    async fn boom() -> Result<impl View> {
        std::fs::read_to_string("no-such-file.txt")?;
        Ok(view! { <p>"読めました"</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
