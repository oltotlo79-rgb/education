//! ログイン・ログアウトと、「今だれがログインしているか」
//!
//! - パスワードは、Argon2 で「元に戻せない形（ハッシュ）」にして保存する。そのままは保存しない。
//! - ログインすると、Topcoat が推測できない文字列（トークン）を作り、クッキーでブラウザに渡す。
//!   データベースには、その指紋（SHA-256）と期限だけを入れる（sessions の表）。
//! - 最後に使ってから 60 分で、ログインが切れる（使うたびに期限を 60 分先へ延ばす）。

use std::{sync::LazyLock, time::Duration};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use time::OffsetDateTime;
use topcoat::{
    context::{Cx, memoize},
    router::{error::see_other, href},
    session::{self, SessionConfig},
};

use crate::{models::Employee, state::state};

/// 何も操作しないと、この時間でログインが切れる
pub const IDLE: Duration = Duration::from_secs(60 * 60);

/// ルーターに渡す、セッションの設定（HTTP 用のクッキー・60 分）
pub fn session_config() -> SessionConfig {
    SessionConfig::builder().token_store(crate::http_cookie::HttpCookieTokenStore).lifetime(IDLE).build()
}

// ---------- パスワード ----------

/// パスワードをハッシュにする。わざと重い計算なので、別のスレッドで動かす（ほかの人の画面を止めない）
pub async fn hash_password(password: String) -> String {
    tokio::task::spawn_blocking(move || hash_password_blocking(&password)).await.expect("ハッシュの計算が止まりました")
}

fn hash_password_blocking(password: &str) -> String {
    // 塩（salt）は、毎回ちがう乱数を Argon2 が自分で作り、結果の文字列の中に入れる
    Argon2::default().hash_password(password.as_bytes()).expect("ハッシュにできません").to_string()
}

/// パスワードが合っているか。合っていなければ false
pub async fn verify_password(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        let Ok(parsed) = PasswordHash::new(&hash) else { return false };
        Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
    })
    .await
    .unwrap_or(false)
}

/// 社員がいないときにも、同じ重さの確かめをするための、にせのハッシュ。
/// 「いない社員は早く断られる」と、時間の差で社員番号を探られてしまうので、それを防ぐ
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| hash_password_blocking("dummy-password-for-timing"));

// ---------- ログイン・ログアウト ----------

/// 社員番号とパスワードを確かめて、合っていればログインさせる。合っていなければ Ok(false)
pub async fn login(cx: &Cx, employee_no: &str, password: &str) -> topcoat::Result<bool> {
    let pool = &state(cx).pool;
    let row: Option<(i64, Option<String>)> =
        sqlx::query_as("SELECT id, password_hash FROM employees WHERE employee_no = $1 AND is_active")
            .bind(employee_no.trim())
            .fetch_optional(pool)
            .await?;

    let (id, hash) = match row {
        Some((id, Some(hash))) => (Some(id), hash),
        _ => (None, DUMMY_HASH.clone()), // いない・パスワードが無い人も、同じだけ時間をかける
    };
    let ok = verify_password(password.to_string(), hash).await;
    let Some(employee_id) = id.filter(|_| ok) else {
        return Ok(false);
    };

    // 新しいトークンを作って、クッキーで渡す（前のトークンは使わない。セッション固定の攻撃を防ぐ）
    let s = session::start(cx).await?;
    sqlx::query("INSERT INTO sessions (token_hash, employee_id, expires_at) VALUES ($1, $2, $3)")
        .bind(&s.token_hash[..])
        .bind(employee_id)
        .bind(OffsetDateTime::from(s.expires_at))
        .execute(pool)
        .await?;
    // ついでに、期限の切れた記録を掃除する
    sqlx::query("DELETE FROM sessions WHERE expires_at < now()").execute(pool).await?;
    Ok(true)
}

/// ログアウト。ブラウザのクッキーを消し、データベースの記録も消す
pub async fn logout(cx: &Cx) -> topcoat::Result<()> {
    if let Some(hash) = session::stop(cx).await? {
        sqlx::query("DELETE FROM sessions WHERE token_hash = $1").bind(&hash[..]).execute(&state(cx).pool).await?;
    }
    Ok(())
}

// ---------- 今だれがログインしているか ----------

/// 今ログインしている社員。いなければ None。
/// `#[memoize]` で、1 回のリクエストの中では、何回呼んでもデータベースを 1 回しか見ない
#[memoize(as_ref)]
pub async fn current_user(cx: &Cx) -> topcoat::Result<Option<Employee>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    let user = sqlx::query_as::<_, Employee>(
        "SELECT e.id, e.employee_no, e.name, e.dept, e.job_title, e.email, e.role
           FROM sessions s JOIN employees e ON e.id = s.employee_id
          WHERE s.token_hash = $1 AND s.expires_at > now() AND e.is_active",
    )
    .bind(&hash[..])
    .fetch_optional(&state(cx).pool)
    .await?;
    Ok(user)
}

/// ログインが要るページの、最初に呼ぶ。ログインしていなければ、ログインの画面へ移す。
/// ログインしていれば、期限を 60 分先へ延ばす（クッキーとデータベースの両方）
pub async fn require_user(cx: &Cx) -> topcoat::Result<Employee> {
    let Some(user) = current_user(cx).await?.clone() else {
        return Err(see_other(href!(crate::app::login::login_page).resolve(cx)).into());
    };
    if let Some(s) = session::refresh(cx).await? {
        sqlx::query("UPDATE sessions SET expires_at = $2 WHERE token_hash = $1")
            .bind(&s.token_hash[..])
            .bind(OffsetDateTime::from(s.expires_at))
            .execute(&state(cx).pool)
            .await?;
    }
    Ok(user)
}
