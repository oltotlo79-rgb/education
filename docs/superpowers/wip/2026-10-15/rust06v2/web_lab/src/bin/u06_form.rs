//! 第6巻 UNIT 06: フォームを受け取り、成功したら 303 で移る（PRG）。失敗したら 400 で出し直す。
//! 実行: cargo run --bin u06_form

use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, module_router, page},
    view::{View, component, view},
};

/// 保存したメモ（この例では、プログラムが動いている間だけ覚えておく）
static MEMOS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// フォームから送られてくる値（name="title" の入力欄と対応する）
#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

/// 一覧とフォームの画面（部品）。error があれば、入力欄の下に出す
#[component]
async fn memo_page(title: String, error: Option<&'static str>) -> Result<impl View> {
    let memos = MEMOS.lock().unwrap().clone();
    Ok(view! {
        <h1>"メモ"</h1>
        <form method="post" action="/">
            <input name="title" value=(title)>
            if let Some(error) = error {
                <p class="error">(error)</p>
            }
            <button type="submit">"保存"</button>
        </form>
        <ul>
            for memo in memos {
                <li>(memo)</li>
            }
        </ul>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { memo_page(title: String::new(), error: None) })
}

/// POST /（同じモジュールなので同じ URL。メソッドが違うので取り合わない）
#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<MemoForm>) -> Result<impl View> {
    let title = input.title.trim();
    if title.is_empty() {
        // 失敗: 400 で、入力を残したまま出し直す
        return Ok(view! {
            (StatusCode::BAD_REQUEST)
            memo_page(title: input.title.clone(), error: Some("タイトルを入力してください"))
        });
    }
    MEMOS.lock().unwrap().push(title.to_string());
    // 成功: 303 で一覧（GET /）へ移る
    Err(see_other(href!(home).resolve(cx)).into())
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
