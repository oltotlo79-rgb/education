//! 第6巻 UNIT 05 の例5-2: 利用者が書いた文字を、置く場所ごとにどうエスケープするかを確かめる。
//! 実行: cargo run --bin u05_escape

use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

/// GET /: 危ない文字を含むタイトルを、本文と属性の両方に置く
#[page]
async fn home() -> Result<impl View> {
    // 利用者がフォームに書いた、と考える文字
    let title = r#"<script>alert("x")</script> & 'quote'"#;
    Ok(view! {
        <p>(title)</p>
        <a href="/notes" title=(title)>"一覧"</a>
    })
}

mod name {
    use topcoat::{
        Result,
        context::Cx,
        router::{page, query_params},
        view::{View, view},
    };

    #[query_params]
    struct NameQuery {
        attr: Option<String>,
    }

    /// GET /name?attr=...: 属性の「名前」を、利用者の文字から作ってしまう（してはいけない例）
    #[page]
    async fn attribute_name(cx: &Cx) -> Result<impl View> {
        let attr = query_params::<NameQuery>(cx)
            .ok()
            .and_then(|q| q.attr.clone())
            .unwrap_or_else(|| "data-x".to_string());
        Ok(view! { <p (attr)="1">"属性の名前を試す"</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
