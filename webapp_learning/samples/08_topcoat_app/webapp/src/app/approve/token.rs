//! /approve/{token} … メールの URL から開く、承認の画面。ログインは要らない
//!
//! URL の中の推測できない文字列が、「この回覧者本人だ」という印になる（06 の approve_views.py と同じ考え方）。
//! データベースには文字列そのものではなく指紋だけがあるので、データベースを見られても URL は作れない。

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{
        StatusCode,
        content::Form,
        error::{RouterErrorExt, not_found},
        href, module_param, page, path_param, response::Response, route,
    },
    view::{View, view},
};

use crate::{
    files,
    queries::{self, FullRequest},
    state::state,
    views::request_table,
    workflow::{self, TokenStep, WorkflowError},
};

module_param!(pub token: String, error = not_found);
// 添付ファイルの番号（/approve/{token}/files/{file_id}）
path_param!(pub file_id: i64, error = not_found);

/// URL の文字列を調べる（鍵はかけない。見るだけ）
async fn check(cx: &Cx) -> Result<std::result::Result<TokenStep, String>> {
    let token = path_param::<Token>(cx)?;
    let mut conn = state(cx).pool.acquire().await?;
    match workflow::check_token(&mut conn, token, false).await {
        Ok(step) => Ok(Ok(step)),
        Err(WorkflowError::Rule(m)) => Ok(Err(m)),
        Err(e) => Err(e.into()),
    }
}

#[page]
async fn approve_page(cx: &Cx) -> Result<impl View> {
    let token = path_param::<Token>(cx)?.clone();
    let page: std::result::Result<(TokenStep, FullRequest), String> = match check(cx).await? {
        Ok(step) => {
            let full = queries::find_request(&state(cx).pool, step.request_id).await?.ok_or_not_found()?;
            Ok((step, full))
        }
        Err(m) => Err(m),
    };
    // 使えない URL なら 410 Gone（もう無い）
    let status = if page.is_ok() { StatusCode::OK } else { StatusCode::GONE };
    Ok(view! {
        (status)
        match &page {
            Ok((step, full)) => {
                <h1>"承認のお願い（回覧者" (step.step_order) "）"</h1>
                <p>(&step.approver_name) " さん、次の依頼の承認をお願いします。"</p>
                request_table(r: &full.request)
                if !full.attachments.is_empty() {
                    <h2>"添付ファイル"</h2>
                    <ul>
                        for a in &full.attachments {
                            <li><a href=(href!(approve_file, Token(token.clone()), FileId(a.id)))>(&a.original_name)</a></li>
                        }
                    </ul>
                }
                <h2>"どちらかを選んでください"</h2>
                <form method="post" class="actions left">
                    <button type="submit" name="decision" value="approve" class="btn-primary">"承認する"</button>
                </form>
                <form method="post" class="form-grid">
                    <label for="comment">"差し戻す理由"</label>
                    <textarea id="comment" name="comment" rows="3" required=""></textarea>
                    <span></span>
                    <div class="actions"><button type="submit" name="decision" value="return" class="btn-secondary">"差し戻す"</button></div>
                </form>
            },
            Err(m) => {
                <h1>"承認の画面"</h1>
                <p class="errors">(m)</p>
            },
        }
    })
}

/// ボタンから届く値
#[derive(Deserialize)]
struct DecisionForm {
    decision: String,
    comment: Option<String>,
}

#[page(POST)]
async fn approve_submit(cx: &Cx, Form(input): Form<DecisionForm>) -> Result<impl View> {
    let token = path_param::<Token>(cx)?.clone();
    let result = if input.decision == "approve" {
        workflow::approve(cx, &token).await.map(|t| format!("「{t}」を承認しました。"))
    } else {
        workflow::return_request(cx, &token, input.comment.as_deref().unwrap_or("")).await.map(|t| format!("「{t}」を差し戻しました。"))
    };
    let (status, class, message) = match result {
        Ok(m) => (StatusCode::OK, "message", m),
        Err(WorkflowError::Rule(m)) => (StatusCode::BAD_REQUEST, "errors", m),
        Err(e) => return Err(e.into()),
    };
    Ok(view! {
        (status)
        <h1>"承認の画面"</h1>
        <p class=(class) role="status">(message)</p>
        <p>"この画面は、閉じてかまいません。"</p>
    })
}

/// 承認の画面から、添付ファイルを開く（URL が使える間だけ・その依頼のファイルだけ）
#[route(GET "./files/{file_id}")]
async fn approve_file(cx: &Cx) -> Result<Response> {
    let Ok(step) = check(cx).await? else {
        return Err(not_found().into());
    };
    let file_id = *path_param::<FileId>(cx)?;
    let st = state(cx);
    let a = queries::find_attachment(&st.pool, file_id).await?.ok_or_not_found()?;
    if a.request_id != step.request_id {
        return Err(not_found().into());
    }
    files::download(&st.config.file_dir, &a).await
}
