//! 問2の解答: 無くてもよい値は try_app_context で借りる。値の違う2つのルーターを、1つのプログラムで試す。
//! 実行: cargo run --bin u07_q2

use topcoat::{
    Result,
    context::{Cx, try_app_context},
    router::{Body, Method, Router, module_router, page, request::Request, to_bytes},
    view::{View, view},
};

/// あってもなくてもよい、お知らせの文
struct Banner(String);

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // 登録されていなければ None になる（panic しない）
    let banner = try_app_context::<Banner>(cx).map(|b| b.0.clone());
    Ok(view! {
        if let Some(banner) = banner {
            <p class="banner">(banner)</p>
        }
        <h1>"トップ"</h1>
    })
}

/// サーバーを起動せずに、ルーターに GET / を1回処理させて、本文を返す
async fn get_home(router: &Router) -> String {
    let request = Request::builder()
        .method(Method::GET)
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let response = router.handle(request).await;
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::main]
async fn main() {
    let plain = module_router!().build();
    let with_banner = module_router!()
        .app_context(Banner("メンテナンスのお知らせ".to_string()))
        .build();
    println!("Banner なし: {}", get_home(&plain).await);
    println!("Banner あり: {}", get_home(&with_banner).await);
}
