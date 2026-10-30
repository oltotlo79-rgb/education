//! 第11巻 UNIT 03 問3: 題に HTML の記号を入れたノートが、一覧で「ただの文字」として出るかを、
//! CSS セレクターで確かめる。
//! 実行: cargo test --test u03_q3 -- --nocapture

mod common;

use common::{body_text, test_app};
use scraper::{Html, Selector};

#[tokio::test]
async fn titles_are_text_not_html() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let title = r#"<b>太字</b> & "引用""#;
    app.create_note(&alice, title, "").await;

    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    let document = Html::parse_document(&html);
    let links: Vec<String> = document
        .select(&Selector::parse("ul.note-list li a").unwrap())
        .map(|link| link.text().collect())
        .collect();
    let bold = document
        .select(&Selector::parse("ul.note-list b").unwrap())
        .count();
    let raw_start = html.find("<li class=\"note-item\">").unwrap();
    let raw = &html[raw_start..raw_start + html[raw_start..].find("</a>").unwrap() + 4];

    println!("本文の中の形: {raw}");
    println!("リンクの文字: {links:?}");
    println!("一覧の中の b 要素: {bold} 個");
    assert_eq!(links, [title]);
    assert_eq!(bold, 0);
}
