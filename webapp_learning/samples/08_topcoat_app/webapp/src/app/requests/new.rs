//! /requests/new … 新しい依頼（GET で入力の画面、POST で依頼を出す）

use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::multipart::Multipart, error::see_other, href, page},
    view::{View, view},
};

use super::id::{Id, Msg, detail};
use crate::{
    auth::require_user,
    queries,
    request_form::{FormValues, request_form},
    state::state,
    workflow::{self, WorkflowError},
};

#[page]
pub async fn new_page(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let choices = queries::approver_choices(&state(cx).pool, &user).await?;
    let values = FormValues::default();
    Ok(view! {
        <h1>"新しい依頼"</h1>
        request_form(values: &values, choices: &choices, errors: &[], attachments: &[], submit_label: "依頼を出す")
    })
}

#[page(POST)]
async fn new_submit(cx: &Cx, multipart: Multipart) -> Result<impl View> {
    let user = require_user(cx).await?;
    let values = FormValues::read(multipart).await?;
    // 1. 形を確かめる → 2. 決まりを確かめて保存する。どちらかでだめなら、入れた値のまま画面に戻す
    let errors = match values.validate() {
        Ok(input) => match workflow::submit(cx, &user, input).await {
            // PRG: 保存できたら、303 で詳しい画面（GET）へ。再読み込みで二重に出さない
            Ok(id) => return Err(see_other(href!(detail, Id(id)).query(Msg { msg: "submitted" }).resolve(cx)).into()),
            Err(WorkflowError::Rule(m)) => vec![m],
            Err(e) => return Err(e.into()),
        },
        Err(errors) => errors,
    };
    let choices = queries::approver_choices(&state(cx).pool, &user).await?;
    // ファイルは、もう一度選んでもらう（ブラウザの決まりで、ファイルの欄には値を戻せない）
    let values = FormValues { files: Vec::new(), ..values };
    Ok(view! {
        (StatusCode::BAD_REQUEST)
        <h1>"新しい依頼"</h1>
        request_form(values: &values, choices: &choices, errors: &errors, attachments: &[], submit_label: "依頼を出す")
    })
}
