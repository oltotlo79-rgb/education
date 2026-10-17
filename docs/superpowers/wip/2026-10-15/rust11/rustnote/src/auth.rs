//! 認証（ログイン）まわりの関数。
//!
//! Topcoat の考え方は「ミドルウェアではなく関数」。ログインが必要なページは、
//! 処理の最初で `require_user(cx)` を呼ぶ。呼び忘れが見つけやすく、順番の心配もない。
//!
//! - パスワード … Argon2id でハッシュ化して保存し、照合する（CPU を使う重い処理なので
//!   tokio の spawn_blocking で別のスレッドに任せる）
//! - セッション … Topcoat の `session::start` がトークンを発行してクッキー
//!   （__Host-session、Secure・HttpOnly・SameSite=Lax）に入れる。アプリはトークンの
//!   ハッシュ（TokenHash）を sessions テーブルに保存して、次のリクエストで照合する

use std::sync::LazyLock;
use std::time::SystemTime;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use topcoat::{
    Result,
    context::{Cx, memoize},
    router::{error::see_other, href},
    session,
};

use crate::domain::{Email, Password, User};
use crate::error::AppError;
use crate::flash::{self, Flash};
use crate::state::app_state;

// ---------------------------------------------------------------------------
// パスワード（Argon2id）
// ---------------------------------------------------------------------------

/// パスワードを Argon2id でハッシュ化し、PHC 文字列（$argon2id$v=19$...）にする。
/// 塩（salt）は argon2 クレートが毎回ランダムに作る。
pub async fn hash_password(password: &Password) -> Result<String, AppError> {
    let password = password.clone();
    // 重い計算なので、非同期の処理を止めないよう別スレッドで行う
    let joined = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_str().as_bytes())
            .map(|hash| hash.to_string())
    })
    .await;
    match joined {
        Ok(Ok(phc)) => Ok(phc),
        Ok(Err(error)) => Err(AppError::internal(error)),
        Err(error) => Err(AppError::internal(error)),
    }
}

/// 入力されたパスワードが、保存してあるハッシュと一致するか調べる
pub async fn verify_password(password: &str, phc: &str) -> Result<bool, AppError> {
    let password = password.to_string();
    let phc = phc.to_string();
    let joined = tokio::task::spawn_blocking(move || {
        // 保存してある文字列を読み取る。読めないならデータが壊れている（内部エラー）
        let parsed = PasswordHash::new(&phc)?;
        // 一致しなければ Err が返るので、is_ok() で true / false にする
        Ok::<bool, argon2::password_hash::Error>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
        )
    })
    .await;
    match joined {
        Ok(Ok(matches)) => Ok(matches),
        Ok(Err(error)) => Err(AppError::internal(error)),
        Err(error) => Err(AppError::internal(error)),
    }
}

/// 登録されていないメールでも同じくらい時間をかけるための、ダミーのハッシュ。
/// （すぐ返すと「このメールは未登録だ」と応答時間から推測されてしまう）
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    Argon2::default()
        .hash_password(b"rustnote-dummy-password")
        .expect("ダミーのハッシュを作れません")
        .to_string()
});

/// メールアドレスとパスワードを確かめる。正しければ Some(ユーザー)
pub async fn authenticate(
    cx: &Cx,
    email_input: &str,
    password: &str,
) -> Result<Option<User>, AppError> {
    // 形式の違うメールは「見つからない」と同じ扱いにする
    let Ok(email) = Email::parse(email_input) else {
        verify_password(password, &DUMMY_HASH).await?;
        return Ok(None);
    };
    match app_state(cx).users.find_by_email(&email).await? {
        Some(record) => {
            if verify_password(password, &record.password_hash).await? {
                Ok(Some(record.user))
            } else {
                Ok(None)
            }
        }
        None => {
            verify_password(password, &DUMMY_HASH).await?;
            Ok(None)
        }
    }
}

// ---------------------------------------------------------------------------
// セッション（ログイン・ログアウト）
// ---------------------------------------------------------------------------

/// ログインさせる: 新しいトークンを発行してクッキーに入れ、そのハッシュを DB に記録する
pub async fn log_in(cx: &Cx, user: &User) -> Result<(), AppError> {
    let state = app_state(cx);
    // 前のセッションのクッキーが残っていれば、その記録は先に消しておく
    if let Some(old_hash) = session::token_hash(cx).await.map_err(AppError::internal)? {
        state.sessions.delete(&old_hash).await?;
    }
    // 毎回新しいトークンを作る（セッション固定攻撃を防ぐ）
    let session = session::start(cx).await.map_err(AppError::internal)?;
    state
        .sessions
        .create(&session.token_hash, user.id, session.expires_at)
        .await?;
    // ついでに、期限切れのセッションを掃除しておく
    state.sessions.delete_expired(SystemTime::now()).await?;
    Ok(())
}

/// ログアウトさせる: クッキーを消し、DB の記録も消す
pub async fn log_out(cx: &Cx) -> Result<(), AppError> {
    if let Some(token_hash) = session::stop(cx).await.map_err(AppError::internal)? {
        app_state(cx).sessions.delete(&token_hash).await?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// ログイン中のユーザー
// ---------------------------------------------------------------------------

/// クッキーのトークンから、ログイン中のユーザーを DB で探す。
/// `#[memoize(as_ref)]` により、1つのリクエストの中では1回しか DB に問い合わせない
/// （レイアウトとページの両方から呼ばれても、2回目は覚えておいた結果を返す）。
#[memoize(as_ref)]
async fn load_current_user(cx: &Cx) -> Result<Option<User>, AppError> {
    let Some(token_hash) = session::token_hash(cx).await.map_err(AppError::internal)? else {
        return Ok(None);
    };
    let user = app_state(cx)
        .sessions
        .find_user(&token_hash, SystemTime::now())
        .await?;
    Ok(user)
}

/// ログイン中のユーザー（いなければ None）
pub async fn current_user(cx: &Cx) -> Result<Option<&User>, AppError> {
    match load_current_user(cx).await {
        Ok(user) => Ok(user.as_ref()),
        // 覚えておいたエラーは借り物なので、文章にして新しいエラーを作る
        Err(error) => Err(AppError::internal(error.to_string())),
    }
}

/// ログインが必要なページの最初に呼ぶ。
/// ログインしていなければ「ログインしてください」を予約し、303 でログイン画面へ送る。
pub async fn require_user(cx: &Cx) -> Result<&User> {
    match current_user(cx).await? {
        Some(user) => Ok(user),
        None => {
            flash::set(cx, Flash::LoginRequired);
            let login_url = href!(crate::routes::login::login_page).resolve(cx);
            Err(see_other(login_url).into())
        }
    }
}
