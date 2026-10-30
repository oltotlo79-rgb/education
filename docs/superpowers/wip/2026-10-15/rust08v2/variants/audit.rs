//! 監査の記録（第8巻 UNIT 10 で、RustNote のコピーに足して試す）。
//!
//! 「いつ・どこから・誰が・何をして・どうなったか」を、`[audit]` で始まる1行にして
//! 標準エラー出力に書く。パスワード・トークン・クッキーの値は、書かない。

use std::time::{SystemTime, UNIX_EPOCH};

use topcoat::{context::Cx, router::request::client_ip};

use crate::domain::UserId;

/// 記録する出来事
pub enum Event<'a> {
    /// ログインできた
    LoginSucceeded(UserId),
    /// ログインに失敗した（入力されたメールアドレス）
    LoginFailed(&'a str),
    /// ログアウトした（ログインしていなかったときは None）
    LoggedOut(Option<UserId>),
}

/// メールアドレスを、先頭の1文字と @ から後ろだけにする（alice@example.com → a…@example.com）。
/// @ が無いときは、何も残さない（メールの欄に、パスワードを打ってしまった人がいるため）
fn mask_email(email: &str) -> String {
    match email.split_once('@') {
        Some((name, domain)) => {
            let first: String = name.chars().take(1).collect();
            format!("{first}…@{domain}")
        }
        None => "（メールアドレスの形ではない）".to_string(),
    }
}

/// 1行の記録を書く
pub fn record(cx: &Cx, event: Event<'_>) {
    // 1970 年 1 月 1 日からの秒数
    let at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0);
    let ip = client_ip(cx).map_or("-".to_string(), |ip| ip.to_string());
    let what = match event {
        Event::LoginSucceeded(user) => format!("login_succeeded user={user}"),
        Event::LoginFailed(email) => format!("login_failed email={}", mask_email(email)),
        Event::LoggedOut(Some(user)) => format!("logged_out user={user}"),
        Event::LoggedOut(None) => "logged_out user=-".to_string(),
    };
    eprintln!("[audit] at={at} ip={ip} {what}");
}
