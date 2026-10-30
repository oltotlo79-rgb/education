//! 第8巻 UNIT 10: この巻で確かめた守りを、1本の「守りのテスト」にまとめる。
//! どれかの守りが外れたら、その守りの名前で止まる。
//! 実行: cargo test --test u10_guards -- --nocapture

mod common;

use common::{TestApp, body_text, encode, session_cookie, test_app};
use topcoat::router::{Body, Method, StatusCode, header, request::Request};

const PASSWORD: &str = "correct horse battery";

async fn login(app: &TestApp, email: &str, password: &str) -> topcoat::router::response::Response {
    let form = format!("email={}&password={}", encode(email), encode(password));
    app.post_form("/login", &form, None).await
}

fn ok(name: &str) {
    println!("✓ {name}");
}

#[tokio::test]
async fn guards() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;

    // UNIT 01・07: ログインしていなければ 303、他人のノートは、どの入口でも 404
    assert_eq!(
        app.get("/notes/1", None).await.status(),
        StatusCode::SEE_OTHER,
        "ログインしていない人は、ログイン画面へ"
    );
    for (path, form) in [
        ("/notes/1", None),
        ("/notes/1/edit", None),
        ("/notes/1/toggle", Some("")),
        ("/notes/1/delete", Some("")),
    ] {
        let response = match form {
            Some(form) => app.post_form(path, form, Some(&bob)).await,
            None => app.get(path, Some(&bob)).await,
        };
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "他人のノートは 404: {path}"
        );
    }
    ok("認可: ログインしていなければ 303、他人のノートはどの入口でも 404");

    // UNIT 04: ログインの失敗は、登録の有無で同じ画面
    let a = body_text(login(&app, "alice@example.com", "wrong password").await).await;
    let b = body_text(login(&app, "dave@example.com", "wrong password").await).await;
    assert_eq!(
        a.replace("alice@example.com", "…"),
        b.replace("dave@example.com", "…"),
        "ログインの失敗の画面は、登録の有無で同じ"
    );
    ok("ログイン: 登録の無いメールも、違うパスワードも、同じ画面");

    // UNIT 04: ログインのたびにトークンが新しくなり、前のトークンは使えない
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode(PASSWORD)
    );
    let renewed = session_cookie(&app.post_form("/login", &form, Some(&alice)).await).unwrap();
    assert_ne!(renewed, alice, "ログインのたびに、新しいトークン");
    assert_eq!(
        app.get("/notes", Some(&alice)).await.status(),
        StatusCode::SEE_OTHER,
        "前のトークンは使えない"
    );
    ok("セッション: ログインのたびに新しいトークン、前のトークンは使えない");

    // UNIT 05: ログアウトで DB の記録が消え、写し取ったクッキーも使えない
    let copied = renewed.clone();
    app.post_form("/logout", "", Some(&renewed)).await;
    assert_eq!(
        app.get("/notes", Some(&copied)).await.status(),
        StatusCode::SEE_OTHER,
        "ログアウトのあと、写し取ったクッキーは使えない"
    );
    ok("ログアウト: 写し取ったクッキーも使えない");

    // UNIT 06: ほかのサイトからの POST は 403
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, bob.as_str())
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("sec-fetch-site", "cross-site")
        .body(Body::from("title=x&body="))
        .unwrap();
    assert_eq!(
        app.router.handle(request).await.status(),
        StatusCode::FORBIDDEN,
        "ほかのサイトからの POST は 403"
    );
    ok("CSRF: ほかのサイトからの POST は 403");
}
