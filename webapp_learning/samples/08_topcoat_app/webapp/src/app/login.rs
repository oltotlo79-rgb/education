//! /login … ログイン（GET で画面、POST で確かめる）

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page},
    view::{View, component, view},
};

use crate::auth::{self, current_user};

/// フォームから届く値（input の name と同じ名前）
#[derive(Deserialize)]
pub struct LoginForm {
    employee_no: String,
    password: String,
}

#[component]
async fn login_form(employee_no: &str, error: Option<&str>) -> Result<impl View> {
    Ok(view! {
        <h1>"ログイン"</h1>
        if let Some(e) = error {
            <p class="errors" role="alert">(e)</p>
        }
        <form method="post" class="form-grid">
            <label for="employee_no">"従業員番号"</label>
            <input id="employee_no" name="employee_no" required="" autocomplete="username" inputmode="numeric" value=(employee_no)>
            <label for="password">"パスワード"</label>
            <input id="password" name="password" type="password" required="" autocomplete="current-password">
            <span></span>
            <div class="actions"><button type="submit" class="btn-primary">"ログイン"</button></div>
        </form>
        <p class="hint">"何も操作しないと、60 分でログインが切れます。"</p>
    })
}

#[page]
pub async fn login_page(cx: &Cx) -> Result<impl View> {
    // もうログインしていれば、一覧へ
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(crate::app::home).resolve(cx)).into());
    }
    Ok(view! { login_form(employee_no: "", error: None) })
}

#[page(POST)]
async fn login_submit(cx: &Cx, Form(input): Form<LoginForm>) -> Result<impl View> {
    if auth::login(cx, &input.employee_no, &input.password).await? {
        // PRG: 成功したら 303 で一覧（GET）へ
        return Err(see_other(href!(crate::app::home).resolve(cx)).into());
    }
    // どちらが違ったかは言わない（社員番号が正しいかどうかを、探られないように）
    Ok(view! {
        (StatusCode::UNAUTHORIZED)
        login_form(employee_no: &input.employee_no, error: Some("従業員番号か、パスワードが違います。"))
    })
}
