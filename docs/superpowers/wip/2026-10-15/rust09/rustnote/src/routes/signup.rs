//! GET /signup : 登録フォーム
//! POST /signup : 登録してログインし、303 でノート一覧へ（入力に問題があればフォームを再表示）
//!
//! | 結果 | ステータス |
//! |---|---|
//! | 成功 | 303 → /notes（フラッシュ「登録しました」） |
//! | メール・パスワードの形式が違う | 400（フォームを再表示） |
//! | メールが登録済み | 409（フォームを再表示） |

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, page},
    view::{View, component, view},
};

use crate::auth::{current_user, hash_password, log_in};
use crate::domain::{Email, Password};
use crate::error::{AppError, RepoError};
use crate::flash::{self, Flash};
use crate::state::app_state;
use crate::views::field_error;

/// フォームから送られてくる値（name 属性と同じ名前）
#[derive(Deserialize)]
pub struct SignupForm {
    email: String,
    password: String,
    password_confirm: String,
}

/// フォームに表示する値とエラー（パスワードは再表示しない）
#[derive(Debug, Clone, Default)]
pub struct SignupFormState {
    email: String,
    email_error: Option<String>,
    password_error: Option<String>,
    password_confirm_error: Option<String>,
}

/// 登録の結果
enum SignupOutcome {
    /// 登録してログインまで済んだ
    Registered,
    /// 入力に問題があった（返すステータスと、フォームに出す内容）
    Rejected(StatusCode, SignupFormState),
}

#[component]
async fn signup_form(form: SignupFormState) -> Result<impl View> {
    Ok(view! {
        <h1>"新規登録"</h1>
        <form method="post" action=(href!(signup)) class="auth-form">
            <label for="email">"メールアドレス"</label>
            <input
                id="email"
                name="email"
                type="email"
                value=(form.email)
                required=""
                autocomplete="email"
            >
            field_error(message: form.email_error)
            <label for="password">"パスワード（8文字以上）"</label>
            <input
                id="password"
                name="password"
                type="password"
                required=""
                autocomplete="new-password"
            >
            field_error(message: form.password_error)
            <label for="password_confirm">"パスワード（確認）"</label>
            <input
                id="password_confirm"
                name="password_confirm"
                type="password"
                required=""
                autocomplete="new-password"
            >
            field_error(message: form.password_confirm_error)
            <button type="submit">"登録する"</button>
        </form>
        <p>
            "登録済みの方は "
            <a href=(href!(crate::routes::login::login_page))>"ログイン"</a>
        </p>
    })
}

#[page]
pub async fn signup_page(cx: &Cx) -> Result<impl View> {
    // ログイン済みなら登録画面は不要
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
    }
    Ok(view! { signup_form(form: SignupFormState::default()) })
}

#[page(POST)]
pub async fn signup(cx: &Cx, Form(input): Form<SignupForm>) -> Result<impl View> {
    let (status, form) = match register(cx, &input).await? {
        SignupOutcome::Registered => {
            flash::set(cx, Flash::SignedUp);
            // PRG: 成功したら 303 で GET の画面へ移る
            return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
        }
        SignupOutcome::Rejected(status, form) => (status, form),
    };
    Ok(view! {
        (status)
        signup_form(form: form)
    })
}

/// 入力を検査し、ユーザーを登録してログインさせる
async fn register(cx: &Cx, input: &SignupForm) -> Result<SignupOutcome, AppError> {
    let mut form = SignupFormState {
        email: input.email.clone(),
        ..SignupFormState::default()
    };

    // 1) 入力の検査（全部の問題を一度に出せるよう、途中で止めない）
    let email = match Email::parse(&input.email) {
        Ok(email) => Some(email),
        Err(error) => {
            form.email_error = Some(error.to_string());
            None
        }
    };
    let password = match Password::parse(&input.password) {
        Ok(password) => Some(password),
        Err(error) => {
            form.password_error = Some(error.to_string());
            None
        }
    };
    if input.password != input.password_confirm {
        form.password_confirm_error = Some("確認用のパスワードが一致しません".to_string());
    }
    let (Some(email), Some(password), None) = (email, password, &form.password_confirm_error)
    else {
        return Ok(SignupOutcome::Rejected(StatusCode::BAD_REQUEST, form));
    };

    // 2) パスワードをハッシュ化して保存する（生のパスワードは保存しない）
    let password_hash = hash_password(&password).await?;
    let user = match app_state(cx).users.create(&email, &password_hash).await {
        Ok(user) => user,
        // users.email の UNIQUE に引っかかった ＝ 登録済みのメール
        Err(RepoError::UniqueViolation(_)) => {
            form.email_error = Some(AppError::EmailTaken.to_string());
            return Ok(SignupOutcome::Rejected(StatusCode::CONFLICT, form));
        }
        Err(other) => return Err(other.into()),
    };

    // 3) そのままログインさせる
    log_in(cx, &user).await?;
    Ok(SignupOutcome::Registered)
}
