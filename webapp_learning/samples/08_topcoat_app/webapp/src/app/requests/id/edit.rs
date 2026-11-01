//! /requests/{id}/edit … 直して出し直す（差し戻し中・引き戻し中の、自分の依頼だけ）

use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::multipart::Multipart, error::see_other, href, page, path_param},
    view::{View, view},
};

use super::{Id, Msg, detail, load_visible};
use crate::{
    auth::require_user,
    models::{Employee, Status},
    queries::{self, FullRequest},
    request_form::{FormValues, request_form},
    state::state,
    workflow::{self, WorkflowError},
};

/// 直せる依頼だけを読む（自分の・差し戻し中か引き戻し中）
async fn load_editable(cx: &Cx, user: &Employee) -> Result<FullRequest> {
    let id = *path_param::<Id>(cx)?;
    let full = load_visible(cx, user, id).await?;
    let r = &full.request;
    if r.requester_id != user.id || !matches!(r.status, Status::Returned | Status::PulledBack) {
        // 直せない依頼なら、詳しい画面へ戻す
        return Err(see_other(href!(detail, Id(id)).resolve(cx)).into());
    }
    Ok(full)
}

#[page]
pub async fn edit_page(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let full = load_editable(cx, &user).await?;
    // 前の回の回覧者を、はじめから選んでおく
    let approver_ids: Vec<i64> = full.current_steps().map(|s| s.approver_id).collect();
    let values = FormValues::from_request(&full.request, &approver_ids);
    let choices = queries::approver_choices(&state(cx).pool, &user).await?;
    Ok(view! {
        <h1>"依頼 " (full.request.id) " を直して出し直す"</h1>
        if !full.request.return_comments.is_empty() {
            <p class="errors pre">(&full.request.return_comments)</p>
        }
        request_form(values: &values, choices: &choices, errors: &[], attachments: &full.attachments, submit_label: "出し直す")
    })
}

#[page(POST)]
async fn edit_submit(cx: &Cx, multipart: Multipart) -> Result<impl View> {
    let user = require_user(cx).await?;
    let full = load_editable(cx, &user).await?;
    let id = full.request.id;
    let values = FormValues::read(multipart).await?;
    let errors = match values.validate() {
        Ok(input) => match workflow::resubmit(cx, &user, id, input).await {
            Ok(()) => return Err(see_other(href!(detail, Id(id)).query(Msg { msg: "resubmitted" }).resolve(cx)).into()),
            Err(WorkflowError::Rule(m)) => vec![m],
            Err(e) => return Err(e.into()),
        },
        Err(errors) => errors,
    };
    let choices = queries::approver_choices(&state(cx).pool, &user).await?;
    let values = FormValues { files: Vec::new(), ..values };
    Ok(view! {
        (StatusCode::BAD_REQUEST)
        <h1>"依頼 " (id) " を直して出し直す"</h1>
        request_form(values: &values, choices: &choices, errors: &errors, attachments: &full.attachments, submit_label: "出し直す")
    })
}
