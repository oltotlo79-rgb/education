//! 第10巻 UNIT 03: 詳細の画面の HTML から、ノートの部分（<article>）と、
//! 次の操作への入り口（編集・完了・削除・一覧へ）を抜き出して表示する。
//! 実行: cargo test --test u03_page -- --nocapture

mod common;

use common::{body_text, test_app};

/// HTML から、start で始まり end で終わる最初の部分を取り出す
fn between<'a>(html: &'a str, start: &str, end: &str) -> &'a str {
    let from = html.find(start).expect("始まりが見つかりません");
    let to = html[from..].find(end).expect("終わりが見つかりません") + from + end.len();
    &html[from..to]
}

#[tokio::test]
async fn detail_page_parts() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let url = app
        .create_note(&alice, "<b>牛乳</b>を買う", "1本\n低脂肪 & 冷やす")
        .await;
    let html = body_text(app.get(&url, Some(&alice)).await).await;

    println!("--- ノートの部分 ---");
    println!("{}", between(&html, "<article", "</article>"));
    println!("--- 次の操作への入り口 ---");
    let actions = between(&html, "<div class=\"note-actions\">", "</div>");
    for part in actions.split("><") {
        if part.starts_with("a ") || part.starts_with("form ") {
            println!("<{part}>");
        }
    }
    let back = between(&html, "<p><a href=\"/notes\"", "</p>");
    println!("{back}");
}
