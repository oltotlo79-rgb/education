//! 第11巻 UNIT 03: 本文の確かめ方を比べる。
//! 「本文に、その文字があるか」（contains）と、「一覧のリンクに、その題があるか」（scraper の CSS セレクター）。
//!
//! 実行: cargo test --test u03_body -- --nocapture

mod common;

use common::{body_text, test_app};
use scraper::{Html, Selector};

const PASSWORD: &str = "correct horse battery";

/// 一覧のページの、ノートのリンクの文字を、上から順に返す
fn listed_titles(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let links = Selector::parse("ul.note-list li a").unwrap();
    document
        .select(&links)
        .map(|link| link.text().collect::<String>())
        .collect()
}

#[tokio::test]
async fn search_by_contains() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "本を返す", "").await;

    let html = body_text(app.get("/notes?q=%E8%B2%B7%E3%81%86", Some(&alice)).await).await;
    println!("contains(\"買う\") → {}", html.contains("買う"));
    assert!(
        html.contains("買う"),
        "「買う」で検索すると、本文に「買う」がある"
    );
}

#[tokio::test]
async fn search_by_selector() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "本を返す", "").await;

    let html = body_text(app.get("/notes?q=%E8%B2%B7%E3%81%86", Some(&alice)).await).await;
    let titles = listed_titles(&html);
    println!("一覧のリンク → {titles:?}");
    assert_eq!(
        titles,
        ["牛乳を買う"],
        "「買う」で検索すると、一覧は「牛乳を買う」だけ"
    );
}
