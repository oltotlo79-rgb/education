//! 第8巻 UNIT 10: 「攻撃されたかもしれない」と思ったときに、あとから読める記録が残るかを試す。
//! 攻撃する人（203.0.113.7）と alice（198.51.100.20）のリクエストを順に送り、
//! サーバーが標準エラー出力に書いた行を、そのまま見る。
//!
//! 実行: cargo test --test u10_audit -- --nocapture

mod common;

use common::{encode, session_cookie, test_app};
use topcoat::router::{
    Body, Method, RemoteAddr, Router, header, request::Request, response::Response,
};

const PASSWORD: &str = "correct horse battery";
const ATTACKER: &str = "203.0.113.7:51000";
const ALICE: &str = "198.51.100.20:62000";

/// どこから来たか（接続の相手のアドレス）を付けて送る
async fn send(
    router: &Router,
    from: &str,
    path: &str,
    form: &str,
    cookie: Option<&str>,
    cross_site: bool,
) -> Response {
    let mut builder = Request::builder()
        .method(Method::POST)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .extension(RemoteAddr(from.parse().unwrap()));
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    if cross_site {
        builder = builder.header("sec-fetch-site", "cross-site");
    }
    router
        .handle(builder.body(Body::from(form.to_string())).unwrap())
        .await
}

fn login_form(email: &str, password: &str) -> String {
    format!("email={}&password={}", encode(email), encode(password))
}

#[tokio::test]
async fn what_is_left_in_the_log() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    let router = &app.router;
    eprintln!("=== ここから ===");

    eprintln!("--- 1. 攻撃する人が、alice のパスワードを 2 回まちがえる");
    for guess in ["password", "123456"] {
        send(
            router,
            ATTACKER,
            "/login",
            &login_form("alice@example.com", guess),
            None,
            false,
        )
        .await;
    }

    eprintln!("--- 2. alice が、メールの欄にパスワードを打ってしまう");
    send(
        router,
        ALICE,
        "/login",
        &login_form(PASSWORD, ""),
        None,
        false,
    )
    .await;

    eprintln!("--- 3. alice がログインする");
    let response = send(
        router,
        ALICE,
        "/login",
        &login_form("alice@example.com", PASSWORD),
        None,
        false,
    )
    .await;
    let alice = session_cookie(&response).unwrap();

    eprintln!("--- 4. ほかのサイトのページが、alice のブラウザに POST /notes を送らせる");
    let response = send(router, ALICE, "/notes", "title=x&body=", Some(&alice), true).await;
    eprintln!("    （返事は {}）", response.status());

    eprintln!("--- 5. alice がログアウトする");
    send(router, ALICE, "/logout", "", Some(&alice), false).await;
    eprintln!("=== ここまで ===");
}
