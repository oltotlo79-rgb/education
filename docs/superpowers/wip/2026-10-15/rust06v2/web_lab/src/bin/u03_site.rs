//! 第6巻 UNIT 03: モジュールの並びを、そのまま URL の並びにする（module_router!）。
//! 実行: cargo run --bin u03_site

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, href, layout, module_router, page},
    view::{View, class, view},
};

/// すべてのページを包むレイアウト（HTML の外枠と、上のメニュー）
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    // 「この場所について」のリンク先と、いまそのページを開いているか
    let about = href!(about::about_page);
    let on_about = about.is_current(cx);
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 03 の練習"</title>
            </head>
            <body>
                <nav>
                    <a href=(href!(home))>"トップ"</a>
                    <a
                        href=(about)
                        aria-current=(on_about.then_some("page"))
                        class=(class!("nav", "active" if on_about))
                    >
                        "この場所について"
                    </a>
                    <a href=(href!(blog_posts::list))>"記事"</a>
                </nav>
                (slot)
            </body>
        </html>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    /// GET /about
    #[page]
    pub async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }
}

mod blog_posts {
    use super::*;

    /// GET /blog-posts（モジュール名の _ は - になる）
    #[page]
    pub async fn list() -> Result<impl View> {
        Ok(view! { <h1>"記事の一覧"</h1> })
    }
}

mod api {
    mod health {
        use topcoat::{Result, router::route};

        /// GET /api/health（HTML ではなく、文字だけを返す）
        #[route(GET)]
        async fn health() -> Result<&'static str> {
            Ok("ok")
        }
    }
}

#[tokio::main]
async fn main() {
    // このファイル（クレートの一番上）が / になる
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
