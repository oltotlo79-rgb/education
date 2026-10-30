//! 第10巻 UNIT 07: 一覧の検索（GET /notes?q=…）で、% や _ や \ を、ただの文字として探せるかを確かめる。
//! 実行: cargo test --test u07_search -- --nocapture

mod common;

use common::{body_text, encode, test_app};

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

/// 検索の入力欄に入っている値（value 属性）。無ければ "（なし）"
fn search_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"q\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_else(|| "（なし）".to_string())
}

/// 「1 / 1 ページ（全3件）」のような、ページの行
fn page_line(html: &str) -> String {
    html.split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("<span>").nth(1))
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("（なし）")
        .to_string()
}

#[tokio::test]
async fn search_special_characters() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for (title, body) in [
        ("セール 100% オフ", ""),
        ("100 円の牛乳", ""),
        ("file_name の決め方", ""),
        ("file-name を決める", ""),
        (r"C:\tmp の掃除", ""),
        ("買い物メモ", "牛乳と卵"),
    ] {
        app.create_note(&alice, title, body).await;
    }
    for q in [
        "100%",
        "file_name",
        r"\",
        "牛乳",
        "%",
        "_",
        "  牛乳  ",
        "   ",
    ] {
        let url = format!("/notes?q={}", encode(q));
        let html = body_text(app.get(&url, Some(&alice)).await).await;
        println!("q={q:?}");
        println!("   見つかった: {:?}", titles(&html));
        println!(
            "   入力欄の値: {:?}・{}",
            search_value(&html),
            page_line(&html)
        );
    }
}
