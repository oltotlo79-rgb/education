//! いちばん小さな依頼・承認アプリ（Topcoat 0.11）。データはメモリーに持つ（再起動で消える）
//!
//! | 関数 | URL |
//! |---|---|
//! | `home`   | GET /  … 依頼の一覧と、入力フォーム |
//! | `create` | POST / … 依頼を足して、303 で GET / へ（PRG） |

use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::{Cx, app_context},
    router::{Router, Slot, content::Form, error::see_other, href, layout, module_router, page},
    view::{View, view},
};

use crate::features::{Request, Store};

/// ルーターを組み立てる。モジュールの並びが、そのまま URL の並びになる
pub fn router() -> Router {
    module_router!()
        .assets(AssetBundle::load().unwrap())
        .app_context(Mutex::new(Store::sample())) // どのページからも使える、共有のデータ
        .build()
}

fn store(cx: &Cx) -> &Mutex<Store> {
    app_context::<Mutex<Store>>(cx)
}

#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"依頼・承認アプリ（Topcoat の練習）"</title>
                <link rel="stylesheet" href=(asset!("styles.css"))>
                topcoat::dev::script()
            </head>
            <body>
                <header class="site-header"><strong>"依頼・承認アプリ"</strong></header>
                <main>(slot)</main>
            </body>
        </html>
    })
}

/// フォームから送られてくる値（input の name と同じ名前）
#[derive(Deserialize)]
pub struct NewRequestForm {
    title: String,
    due_date: String,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // ロックしている間に、表示に要る物だけ写し取る（ロックを長く持たない）
    let rows: Vec<Request> = store(cx).lock().unwrap().requests.clone();
    Ok(view! {
        <h1>"依頼の一覧"</h1>
        <table>
            <thead><tr><th>"番号"</th><th>"件名"</th><th>"希望の納期"</th><th>"状態"</th></tr></thead>
            <tbody>
                for r in rows {
                    <tr>
                        <td>(r.id)</td>
                        <td>(r.title)</td>
                        <td>(r.due_date)</td>
                        <td><span class=(format!("badge {}", r.status.css_class()))>(r.status.label())</span></td>
                    </tr>
                }
            </tbody>
        </table>

        <h2>"新しい依頼"</h2>
        <form method="post" action=(href!(create)) class="form-grid">
            <label for="title">"件名"</label>
            <input id="title" name="title" required="" maxlength="100">
            <label for="due">"希望の納期"</label>
            <input id="due" name="due_date" type="date" required="">
            <span></span>
            <div><button type="submit" class="btn-primary">"依頼を足す"</button></div>
        </form>
    })
}

#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<NewRequestForm>) -> Result<impl View> {
    let title = input.title.trim();
    if !title.is_empty() {
        store(cx).lock().unwrap().add(title, &input.due_date);
    }
    // PRG: 足したら 303 で一覧（GET）へ。再読み込みで二重に足されない
    Err::<(), _>(see_other(href!(home).resolve(cx)).into())
}
