//! 第10巻 UNIT 07: bob が、検索のキーワードに ' や SQL の断片を入れたら、どうなるかを確かめる。
//! 実行: cargo test --test u07_inject -- --nocapture

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

#[tokio::test]
async fn keyword_with_sql() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "alice の給料の明細", "").await;
    app.create_note(&alice, "alice の病院の予約", "").await;
    app.create_note(&bob, "bob の買い物", "").await;

    for q in ["買い物", "'", "' OR 1=1) OR (title LIKE '"] {
        let response = app
            .get(&format!("/notes?q={}", encode(q)), Some(&bob))
            .await;
        let status = response.status();
        let html = body_text(response).await;
        println!("bob が q={q:?} で検索 → {status}");
        println!("   見つかった: {:?}", titles(&html));
    }
}
