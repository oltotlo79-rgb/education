//! 第10巻 UNIT 07 練習問題。
//! 実行: cargo test --test u07_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, encode, test_app};

const PASSWORD: &str = "correct horse battery";

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

/// 検索の入力欄に入っている値（value 属性）
fn search_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"q\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_default()
}

/// 問1: 大文字と小文字、全角と半角
#[tokio::test]
async fn q1_case_and_width() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for title in ["Rust 入門", "ｒｕｓｔ の本", "Äpfel kaufen"] {
        app.create_note(&alice, title, "").await;
    }
    for q in ["rust", "RUST", "ｒｕｓｔ", "äpfel", "Äpfel"] {
        let html = body_text(
            app.get(&format!("/notes?q={}", encode(q)), Some(&alice))
                .await,
        )
        .await;
        println!("問1 q={q:<6} → {:?}", titles(&html));
    }
}

/// 問2: とても長いキーワード
#[tokio::test]
async fn q2_long_keyword() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let keyword = "あ".repeat(101);
    let html = body_text(
        app.get(&format!("/notes?q={}", encode(&keyword)), Some(&alice))
            .await,
    )
    .await;
    println!(
        "問2 送ったキーワード: {} 文字 → 入力欄の値: {} 文字",
        keyword.chars().count(),
        search_value(&html).chars().count()
    );
}

/// 問3: ページの番号がおかしいとき
#[tokio::test]
async fn q3_bad_page_number() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    for url in [
        "/notes?q=%E7%89%9B%E4%B9%B3&page=abc",
        "/notes?q=%E7%89%9B%E4%B9%B3&page=0",
        "/notes?q=%E7%89%9B%E4%B9%B3&page=-1",
    ] {
        let response = app.get(url, Some(&alice)).await;
        let status = response.status();
        let html = body_text(response).await;
        println!(
            "問3 GET {url} → {status}（見つかった: {:?}）",
            titles(&html)
        );
    }
}
