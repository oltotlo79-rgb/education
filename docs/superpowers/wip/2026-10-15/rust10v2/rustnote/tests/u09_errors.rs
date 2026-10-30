//! 第10巻 UNIT 09: RustNote の失敗の返事を、状態コードごとに1つずつ起こし、どこに・何が出るかを並べる。
//! 実行: cargo test --test u09_errors -- --nocapture

mod common;

use common::{body_text, encode, test_app, test_assets};
use rustnote::{config::Config, db::Db, routes, state::AppState};
use topcoat::router::{Body, Method, header, request::Request, response::Response};

/// 返事の本文から、出ている文を1つ読む（入力欄の下・フォームの上・エラーの画面・それ以外）
async fn describe(response: Response) -> String {
    let status = response.status();
    let html = body_text(response).await;
    let pick = |start: &str| {
        html.split(start)
            .nth(1)
            .and_then(|t| t.split("</p>").next())
            .map(str::to_string)
    };
    let (place, text) = if let Some(t) = pick("<p class=\"field-error\">") {
        ("同じ画面・入力欄の下", t)
    } else if let Some(t) = pick("<p class=\"form-notice\">") {
        (
            "同じ画面・フォームの上",
            t.chars().take(24).collect::<String>() + "…",
        )
    } else if html.contains("<section class=\"error-page\">") {
        let t = html
            .split("<section class=\"error-page\">")
            .nth(1)
            .and_then(|t| t.split("<p>").nth(1))
            .and_then(|t| t.split("</p>").next())
            .unwrap_or("")
            .to_string();
        ("エラーの画面", t)
    } else {
        let text = html.trim();
        let text = if text.is_empty() {
            "（本文なし）"
        } else {
            text
        };
        ("Topcoat の短い返事", text.to_string())
    };
    format!("{status}・{place}・{text}")
}

#[tokio::test]
async fn error_summary() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;

    let r = app.post_form("/notes", "title=&body=", Some(&alice)).await;
    println!("入力の間違い（空のタイトル）: {}", describe(r).await);
    let r = app.post_form("/notes", "subject=x", Some(&alice)).await;
    println!("フォームの形の間違い: {}", describe(r).await);
    let r = app.get("/notes/999", Some(&alice)).await;
    println!("無いノート: {}", describe(r).await);
    let form = format!("version=0&title={}&body=", encode("牛乳を2本買う"));
    let r = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    println!("古い version: {}", describe(r).await);
    let r = app.get("/notes/1/delete", Some(&alice)).await;
    println!("GET で削除: {}", describe(r).await);
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes/1/delete")
        .header(header::COOKIE, alice.as_str())
        .header("origin", "https://evil.example")
        .body(Body::empty())
        .unwrap();
    let r = app.router.handle(request).await;
    println!("ほかのサイトからの POST: {}", describe(r).await);

    // DB に届かないルーターで、ログインする（500）
    let config = Config::new("http://127.0.0.1:9".to_string(), None).unwrap();
    let db = Db::open(&config).await.unwrap();
    let broken = common::TestApp {
        router: routes::build_router(AppState::new(db), test_assets()),
        db: common::test_db().await,
    };
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode("correct horse battery")
    );
    let r = broken.post_form("/login", &form, None).await;
    println!("DB に届かない: {}", describe(r).await);
}
