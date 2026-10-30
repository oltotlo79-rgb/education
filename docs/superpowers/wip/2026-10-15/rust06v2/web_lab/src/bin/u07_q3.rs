//! 問3の解答: UNIT 06 の static MEMOS を、app_context で共有する形に直したもの。
//! 実行: cargo run --bin u07_q3

use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{content::Form, error::see_other, href, module_router, page},
    view::{View, view},
};

/// 保存したメモ。ルーターに1つ登録して、どのページからも借りる
struct Memos(Mutex<Vec<String>>);

fn memos(cx: &Cx) -> &Memos {
    app_context::<Memos>(cx)
}

#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let list = memos(cx).0.lock().unwrap().clone();
    Ok(view! {
        <ul>
            for memo in list {
                <li>(memo)</li>
            }
        </ul>
    })
}

#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<MemoForm>) -> Result<()> {
    memos(cx).0.lock().unwrap().push(input.title);
    Err(see_other(href!(home).resolve(cx)).into())
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .app_context(Memos(Mutex::new(Vec::new())))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
