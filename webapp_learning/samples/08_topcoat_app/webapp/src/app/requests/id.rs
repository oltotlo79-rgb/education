//! /requests/{id} … 依頼の詳しい画面と、ボタンの操作（/requests/{id}/action）

mod edit;

use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    context::Cx,
    router::{
        StatusCode,
        content::Form,
        error::{RouterErrorExt, not_found, see_other},
        href, module_param, page, path_param, query_params,
    },
    view::{View, component, view},
};

use crate::{
    auth::require_user,
    models::{Decision, Employee, Role, Status, fmt_datetime, fmt_size},
    queries::{self, FullRequest},
    state::state,
    views::request_table,
    workflow::{self, WorkflowError},
};

// URL の {id} の部分。数でなければ 404
module_param!(pub id: i64, error = not_found);

/// 操作のあとに出す知らせ（?msg=submitted など）。文はこちらで決める（URL の文をそのまま出さない）
#[derive(Serialize)]
pub struct Msg {
    pub msg: &'static str,
}

#[query_params]
struct MsgQuery {
    msg: Option<String>,
}

fn message(key: &str) -> Option<&'static str> {
    Some(match key {
        "submitted" => "依頼を出しました。1 人目の回覧者に、承認のお願いのメールを送りました。",
        "resubmitted" => "出し直しました。1 人目の回覧者から、もう一度回ります。",
        "pull_back" => "引き戻しました。",
        "withdraw" => "取り下げました。",
        "assign" => "担当になりました。",
        "complete" => "完了にしました。依頼者にメールを送りました。",
        "resend" => "承認のお願いを送り直しました（前の URL は使えなくなりました）。",
        _ => return None,
    })
}

/// 見てよい依頼を読む。無い・見てはいけないときは、どちらも 404（あるかどうかも知らせない）
pub(crate) async fn load_visible(cx: &Cx, user: &Employee, id: i64) -> Result<FullRequest> {
    let full = queries::find_request(&state(cx).pool, id).await?.ok_or_not_found()?;
    if !full.visible_to(user) {
        return Err(not_found().into());
    }
    Ok(full)
}

#[page]
pub async fn detail(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    let full = load_visible(cx, &user, id).await?;
    let msg = query_params::<MsgQuery>(cx).ok().and_then(|q| q.msg.as_deref().and_then(message));
    Ok(view! {
        if let Some(m) = msg {
            <p class="message" role="status">(m)</p>
        }
        detail_body(user: &user, full: &full, errors: &[])
    })
}

/// 詳しい画面の中身（操作に失敗したときにも、同じ物を出す）
#[component]
async fn detail_body(user: &Employee, full: &FullRequest, errors: &[String]) -> Result<impl View> {
    let r = &full.request;
    let own = r.requester_id == user.id;
    // 押せるボタンだけを出す（押されたときも、サーバーでもう一度確かめる）
    let can_pull_back = own && r.status == Status::Circulating;
    let can_fix = own && matches!(r.status, Status::Returned | Status::PulledBack);
    let can_assign = user.role.is_staff() && r.status == Status::Approved;
    let can_complete = r.status == Status::InProgress && r.assignee_id == Some(user.id);
    let is_admin = user.role == Role::Admin;
    Ok(view! {
        <h1>"依頼 " (r.id)</h1>
        crate::views::error_list(errors: errors)
        <div class="actions left">
            if can_pull_back {
                <form method="post" action=(href!(action, Id(r.id)))>
                    <button name="action" value="pull_back" class="btn-secondary">"引き戻す"</button>
                </form>
            }
            if can_fix {
                <a class="btn-primary" href=(href!(edit::edit_page, Id(r.id)))>"直して出し直す"</a>
                <form method="post" action=(href!(action, Id(r.id)))>
                    <button name="action" value="withdraw" class="btn-secondary">"取り下げる"</button>
                </form>
            }
            if can_assign {
                <form method="post" action=(href!(action, Id(r.id)))>
                    <button name="action" value="assign" class="btn-primary">"担当になる"</button>
                </form>
            }
            if can_complete {
                <form method="post" action=(href!(action, Id(r.id)))>
                    <button name="action" value="complete" class="btn-primary">"完了にする（引き渡した）"</button>
                </form>
            }
        </div>

        request_table(r: r)

        <h2>"回覧（" (r.round) " 回目）"</h2>
        <ol class="steps">
            for s in full.current_steps() {
                <li class=(format!("step-{}", s.decision.as_str()))>
                    (&s.approver_name) "（" (&s.approver_dept) "）: " (s.decision.label())
                    if let Some(at) = s.decided_at {
                        " " (fmt_datetime(at))
                    }
                    if is_admin && s.decision == Decision::Waiting && r.status == Status::Circulating {
                        " "
                        <form method="post" action=(href!(action, Id(r.id))) style="display:inline">
                            <input type="hidden" name="step_id" value=(s.id.to_string())>
                            <button name="action" value="resend" class="link">"承認のお願いを送り直す"</button>
                        </form>
                    }
                </li>
            }
        </ol>

        <h2>"添付ファイル"</h2>
        if full.attachments.is_empty() {
            <p>"ありません。"</p>
        } else {
            <ul>
                for a in &full.attachments {
                    <li>
                        <a href=(href!(crate::app::attachments::id::download, crate::app::attachments::id::Id(a.id)))>(&a.original_name)</a>
                        " （" (fmt_size(a.size)) "）"
                    </li>
                }
            </ul>
        }

        <h2>"記録"</h2>
        <table>
            <thead><tr><th>"日時"</th><th>"した人"</th><th>"したこと"</th><th>"メモ"</th></tr></thead>
            <tbody>
                for h in &full.histories {
                    <tr>
                        <td>(fmt_datetime(h.at))</td>
                        <td>(h.actor_name.as_deref().unwrap_or("（自動）"))</td>
                        <td>(&h.action)</td>
                        <td class="pre">(&h.note)</td>
                    </tr>
                }
            </tbody>
        </table>
    })
}

/// ボタンから届く値
#[derive(Deserialize)]
struct ActionForm {
    action: String,
    step_id: Option<i64>,
}

#[page(POST "./action")]
async fn action(cx: &Cx, Form(input): Form<ActionForm>) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    load_visible(cx, &user, id).await?; // 見られない依頼は、操作もできない
    let result = match input.action.as_str() {
        "pull_back" => workflow::pull_back(cx, &user, id).await.map(|_| "pull_back"),
        "withdraw" => workflow::withdraw(cx, &user, id).await.map(|_| "withdraw"),
        "assign" => workflow::assign(cx, &user, id).await.map(|_| "assign"),
        "complete" => workflow::complete(cx, &user, id).await.map(|_| "complete"),
        "resend" if user.role == Role::Admin => {
            workflow::resend(cx, Some(&user), input.step_id.unwrap_or_default()).await.map(|_| "resend")
        }
        _ => Err(WorkflowError::Rule("この操作はできません。".into())),
    };
    match result {
        // PRG: 303 で詳しい画面へ戻す
        Ok(key) => Err(see_other(href!(detail, Id(id)).query(Msg { msg: key }).resolve(cx)).into()),
        Err(WorkflowError::Rule(m)) => {
            let full = load_visible(cx, &user, id).await?;
            let errors = vec![m];
            Ok(view! {
                (StatusCode::BAD_REQUEST)
                detail_body(user: &user, full: &full, errors: &errors)
            })
        }
        Err(e) => Err(e.into()),
    }
}
