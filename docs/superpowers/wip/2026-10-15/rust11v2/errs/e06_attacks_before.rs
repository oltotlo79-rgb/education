//! 第11巻 UNIT 06: 守りの回帰テスト。攻撃の種類ごとにテスト関数を分け、
//! 攻撃の入口を表に並べて、全部の入口で止まることを確かめる。
//!
//! 実行: cargo test --test u06_attacks

mod common;

use common::{TestApp, body_text, encode, session_cookie, test_app};
use scraper::{Html, Selector};
use topcoat::router::{Body, Method, StatusCode, header, request::Request, response::Response};

const PASSWORD: &str = "correct horse battery";

/// alice と bob を登録し、alice のノートを1つ作る
async fn two_users() -> (TestApp, String, String) {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    app.create_note(&alice, "alice の秘密", "").await;
    (app, alice, bob)
}

/// フォームを送る。cross_site なら、ほかのサイトのページから送られた形にする
async fn post(app: &TestApp, path: &str, form: &str, cookie: &str, cross_site: bool) -> Response {
    let mut builder = Request::builder()
        .method(Method::POST)
        .uri(path)
        .header(header::COOKIE, cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    if cross_site {
        builder = builder.header("sec-fetch-site", "cross-site");
    }
    app.router
        .handle(builder.body(Body::from(form.to_string())).unwrap())
        .await
}

/// alice のノートが、元のまま残っているか（題と、完了の印）
async fn alice_note_is_intact(app: &TestApp, alice: &str) -> bool {
    let html = body_text(app.get("/notes/1", Some(alice)).await).await;
    html.contains("alice の秘密") && !html.contains(r#"<span class="badge">完了</span>"#)
}

/// 他人のノート（IDOR）: bob からは、どの入口でも 404 で、alice のノートは変わらない
#[tokio::test]
async fn idor_every_entrance() {
    let (app, alice, bob) = two_users().await;
    for path in ["/notes/1", "/notes/1/edit"] {
        let status = app.get(path, Some(&bob)).await.status();
        assert_eq!(status, StatusCode::NOT_FOUND, "IDOR: GET {path}");
    }
    for (path, form) in [
        ("/notes/1/edit", "title=bob&body=&version=1"),
        ("/notes/1/toggle", ""),
        ("/notes/1/delete", ""),
    ] {
        let status = post(&app, path, form, &bob, false).await.status();
        assert_eq!(status, StatusCode::NOT_FOUND, "IDOR: POST {path}");
    }
    assert!(
        alice_note_is_intact(&app, &alice).await,
        "IDOR: alice のノートが変わった"
    );
}

/// ほかのサイトからの POST（CSRF）: 状態を変えるどの入口でも 403 で、何も変わらない
#[tokio::test]
async fn csrf_every_post() {
    let (app, alice, _bob) = two_users().await;
    let signup = format!(
        "email=carol%40example.com&password={0}&password_confirm={0}",
        encode(PASSWORD)
    );
    let login = format!("email=alice%40example.com&password={}", encode(PASSWORD));
    for (path, form) in [
        ("/notes", "title=x&body="),
        ("/notes/1/edit", "title=x&body=&version=1"),
        ("/notes/1/toggle", ""),
        ("/notes/1/delete", ""),
        ("/logout", ""),
        ("/login", login.as_str()),
        ("/signup", signup.as_str()),
    ] {
        let status = post(&app, path, form, &alice, true).await.status();
        assert_eq!(status, StatusCode::FORBIDDEN, "CSRF: POST {path}");
    }
    assert!(
        alice_note_is_intact(&app, &alice).await,
        "CSRF: alice のノートが変わった"
    );
    let status = app.get("/notes", Some(&alice)).await.status();
    assert_eq!(status, StatusCode::OK, "CSRF: alice がログアウトさせられた");
}

/// 保存した HTML（stored XSS）: 題と本文の HTML は、一覧・詳細・編集のどこでも、ただの文字になる
#[tokio::test]
async fn stored_xss_is_text() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let title = r#""><img src=x onerror=alert(1)>"#;
    let body = "<script>alert(2)</script>";
    app.create_note(&alice, title, body).await;

    let injected = Selector::parse("main img, main script").unwrap();
    let value = Selector::parse("input#title").unwrap();
    for path in ["/notes", "/notes/1", "/notes/1/edit"] {
        let html = body_text(app.get(path, Some(&alice)).await).await;
        let document = Html::parse_document(&html);
        let count = document.select(&injected).count();
        assert_eq!(count, 0, "XSS: {path} に img か script の要素ができた");
        if path.ends_with("/edit") {
            let input = document.select(&value).next().unwrap();
            assert_eq!(
                input.value().attr("value"),
                Some(title),
                "XSS: 編集の欄の値が変わった"
            );
        }
    }
}

/// 検索の言葉で、他人のノートが見えない（SQL の注入・LIKE の記号）
#[tokio::test]
async fn search_cannot_cross_owners() {
    let (app, _alice, bob) = two_users().await;
    for keyword in ["%", "_", "' OR 1=1 --", "%' OR user_id != '", "秘密"] {
        let uri = format!("/notes?q={}", encode(keyword));
        let html = body_text(app.get(&uri, Some(&bob)).await).await;
        assert!(
            !html.contains("alice の秘密"),
            "検索: {keyword:?} で alice のノートが見えた"
        );
    }
}

/// セッション: ログインのたびにトークンが変わり、ログアウトしたクッキーは使えない
#[tokio::test]
async fn session_guards() {
    let app = test_app().await;
    let first = app.sign_up("alice@example.com", PASSWORD).await;
    let form = format!("email=alice%40example.com&password={}", encode(PASSWORD));
    let second = session_cookie(&post(&app, "/login", &form, &first, false).await).unwrap();
    assert_ne!(
        first, second,
        "セッション: ログインしてもトークンが変わらない"
    );
    let status = app.get("/notes", Some(&first)).await.status();
    assert_eq!(
        status,
        StatusCode::SEE_OTHER,
        "セッション: 前のトークンが使える"
    );

    post(&app, "/logout", "", &second, false).await;
    let status = app.get("/notes", Some(&second)).await.status();
    assert_eq!(
        status,
        StatusCode::SEE_OTHER,
        "セッション: ログアウトしたクッキーが使える"
    );
}

/// 守りのヘッダー。まだ足していないので、ふだんは動かさない（cargo test -- --ignored で動く）
#[tokio::test]
#[ignore = "改善点: 守りのヘッダーを、まだ返していない（UNIT 03 の問2）"]
async fn security_headers() {
    let app = test_app().await;
    let response = app.get("/login", None).await;
    for name in [
        "x-content-type-options",
        "x-frame-options",
        "content-security-policy",
    ] {
        assert!(
            response.headers().contains_key(name),
            "ヘッダー: {name} が無い"
        );
    }
}
