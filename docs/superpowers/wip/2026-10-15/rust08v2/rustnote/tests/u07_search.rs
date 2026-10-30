//! 第8巻 UNIT 07: bob が、alice のノートの本文にだけある言葉で検索する。
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

#[tokio::test]
async fn search_stays_inside_own_notes() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "銀行のメモ", "暗証番号は 1234")
        .await;
    app.create_note(&bob, "bob の買い物", "牛乳").await;
    for (who, cookie, q) in [
        ("bob", &bob, "暗証番号"),
        ("bob", &bob, "買い物"),
        ("alice", &alice, "暗証番号"),
    ] {
        let html = body_text(
            app.get(&format!("/notes?q={}", encode(q)), Some(cookie))
                .await,
        )
        .await;
        println!("{who} が q={q} で検索 → {:?}", titles(&html));
    }
}
