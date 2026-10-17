//! GET /login : ログインフォーム
//! POST /login : メールとパスワードを確かめ、セッションを始めて 303 でノート一覧へ
//!
//! 失敗したときは、メールが無いのかパスワードが違うのかを区別せず
//! 「メールアドレスまたはパスワードが違います」と 401 で返す（登録の有無を推測させないため）。

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page},
    view::{View, component, view},
};

use crate::auth::{authenticate, current_user, log_in};
use crate::flash::{self, Flash};
use crate::views::flash_banner;

/// ログインに失敗したときの文章
const LOGIN_FAILED: &str = "メールアドレスまたはパスワードが違います";

/// フォームから送られてくる値
#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[component]
async fn login_form(email: String, error: Option<String>) -> Result<impl View> {
    Ok(view! {
        <h1>"ログイン"</h1>
        <form method="post" action=(href!(login)) class="auth-form">
            if let Some(error) = error {
                <p class="form-notice">(error)</p>
            }
            <label for="email">"メールアドレス"</label>
            <input
                id="email"
                name="email"
                type="email"
                value=(email)
                required=""
                autocomplete="email"
            >
            <label for="password">"パスワード"</label>
            <input
                id="password"
                name="password"
                type="password"
                required=""
                autocomplete="current-password"
            >
            <button type="submit">"ログイン"</button>
        </form>
        <p>
            "はじめての方は "
            <a href=(href!(crate::routes::signup::signup_page))>"新規登録"</a>
        </p>
    })
}

#[page]
pub async fn login_page(cx: &Cx) -> Result<impl View> {
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
    }
    // 「ログインしてください」「ログアウトしました」などを表示する
    let flash = flash::take(cx);
    Ok(view! {
        flash_banner(flash: flash)
        login_form(email: String::new(), error: None)
    })
}

#[page(POST)]
pub async fn login(cx: &Cx, Form(input): Form<LoginForm>) -> Result<impl View> {
    if let Some(user) = authenticate(cx, &input.email, &input.password).await? {
        log_in(cx, &user).await?;
        flash::set(cx, Flash::LoggedIn);
        return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
    }
    // メールが無いのか、パスワードが違うのかは区別しない
    let error = Some(LOGIN_FAILED.to_string());
    Ok(view! {
        (StatusCode::UNAUTHORIZED)
        login_form(email: input.email, error: error)
    })
}
