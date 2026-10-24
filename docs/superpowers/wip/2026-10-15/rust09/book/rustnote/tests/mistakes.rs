//! 画面の設計でよくある間違いを、わざと起こして返事を確かめる。
//! 実行: cargo test --test mistakes 名前 -- --nocapture

mod common;

use common::{body_text, test_app};
use topcoat::router::{header, response::Response};

const PASSWORD: &str = "correct horse battery";

// ステータス・Allow ヘッダー・本文の長さと、エラー画面の見出しと説明を表示する
async fn show(request: &str, response: Response) {
    println!("{request} -> {}", response.status());
    if let Some(allow) = response.headers().get(header::ALLOW) {
        println!("    allow: {}", allow.to_str().unwrap());
    }
    let html = body_text(response).await;
    println!("    本文: {} バイト", html.len());
    // エラー画面は <section class="error-page"><h1>…</h1><p>…</p> の形
    if let Some(start) = html.find(r#"<section class="error-page">"#) {
        let section = &html[start..];
        let h1 = between(section, "<h1>", "</h1>");
        let message = between(section, "<p>", "</p>");
        println!("    見出し: {h1}");
        println!("    説明: {message}");
    }
}

// text の中で、最初の begin と、その後の end にはさまれた部分を返す
fn between<'a>(text: &'a str, begin: &str, end: &str) -> &'a str {
    let Some(start) = text.find(begin).map(|i| i + begin.len()) else {
        return "";
    };
    let rest = &text[start..];
    let stop = rest.find(end).unwrap_or(rest.len());
    &rest[..stop]
}

// 間違い1: 「完了にする」をリンク（GET）にしてしまった
#[tokio::test]
async fn toggle_by_link() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&cookie, "牛乳を買う", "").await;

    let path = format!("{url}/toggle");
    show(&format!("GET {path}"), app.get(&path, Some(&cookie)).await).await;
}

// 間違い2: フォームの送り先（action）を /note と書き間違えた
#[tokio::test]
async fn wrong_action() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let response = app.post_form("/note", "title=a&body=", Some(&cookie)).await;
    show("POST /note", response).await;
}

// 間違い3: フォームの項目名を間違えて、body が届かなかった／ページ番号が数字でない
#[tokio::test]
async fn unreadable_input() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let response = app
        .post_form("/notes", "title=a&text=memo", Some(&cookie))
        .await;
    show("POST /notes（body なし）", response).await;
    let response = app.get("/notes?page=abc", Some(&cookie)).await;
    show("GET /notes?page=abc", response).await;
}
