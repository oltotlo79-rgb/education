//! 第10巻 UNIT 08: 55 件のノート（メモ 10 件と、あとから作ったノート 45 件）を、1ページ 20 件で送り、
//! ページごとの中身とリンクを確かめる。キーワードで絞ったときの「次へ」も、たどってみる。
//! 実行: cargo test --test u08_pages -- --nocapture

mod common;

use common::{body_text, test_app};

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

/// ページ送りの部分（<nav class="pagination">）から、ページの行と、前へ・次へのリンク先を読む
fn pagination(html: &str) -> String {
    let nav = html
        .split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("</nav>").next())
        .unwrap_or("");
    let line = nav
        .split("<span>")
        .nth(1)
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("");
    let link = |label: &str| {
        nav.split("<a href=\"")
            .skip(1)
            .find(|t| t.contains(label))
            .and_then(|t| t.split('"').next())
            .map(|href| href.replace("&amp;", "&"))
            .unwrap_or_else(|| "（なし）".to_string())
    };
    format!("{line}　前へ: {}　次へ: {}", link("前へ"), link("次へ"))
}

#[tokio::test]
async fn forty_five_notes() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for n in 1..=10 {
        app.create_note(&alice, &format!("メモ{n:02}"), "").await;
    }
    for n in 1..=45 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    for url in [
        "/notes",
        "/notes?page=2",
        "/notes?page=3",
        "/notes?q=ノート&page=2",
    ] {
        let html = body_text(app.get(url, Some(&alice)).await).await;
        let found = titles(&html);
        println!(
            "GET {url}: {} 件（{} 〜 {}）",
            found.len(),
            found.first().map(String::as_str).unwrap_or("-"),
            found.last().map(String::as_str).unwrap_or("-")
        );
        println!("   {}", pagination(&html));
    }

    // 「ノート」で絞った2ページ目の「次へ」を、そのままたどる
    let html = body_text(app.get("/notes?q=ノート&page=2", Some(&alice)).await).await;
    let next = pagination(&html)
        .split("次へ: ")
        .nth(1)
        .unwrap()
        .to_string();
    let html = body_text(app.get(&next, Some(&alice)).await).await;
    let found = titles(&html);
    println!(
        "「次へ」をたどる GET {next}: {} 件（{} 〜 {}）",
        found.len(),
        found.first().map(String::as_str).unwrap_or("-"),
        found.last().map(String::as_str).unwrap_or("-")
    );
}
