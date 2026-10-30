//! 第10巻 UNIT 03: 詳細の画面（GET /notes/{id}）が、自分のノートだけを 200 で見せ、
//! 無いノート・他人のノート・おかしな ID を、どれも 404 にするかを確かめる。
//! 実行: cargo test --test u03_detail -- --nocapture --test-threads=1

mod common;

use common::{body_text, test_app};

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn detail_answers_by_owner_and_id() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "牛乳を買う", "1本\n低脂肪").await;
    println!("alice のノート: {url}");

    let cases = [
        ("alice", Some(alice.as_str()), "/notes/1"),
        ("bob", Some(bob.as_str()), "/notes/1"),
        ("bob", Some(bob.as_str()), "/notes/999"),
        ("alice", Some(alice.as_str()), "/notes/999"),
        ("alice", Some(alice.as_str()), "/notes/abc"),
        ("alice", Some(alice.as_str()), "/notes/0"),
        ("alice", Some(alice.as_str()), "/notes/-1"),
        ("alice", Some(alice.as_str()), "/notes/9223372036854775808"),
        ("guest", None, "/notes/1"),
    ];
    println!("--- 表 ---");
    for (who, cookie, path) in cases {
        let response = app.get(path, cookie).await;
        let status = response.status();
        let to = response
            .headers()
            .get("location")
            .map(|v| format!(" → {}", v.to_str().unwrap()))
            .unwrap_or_default();
        println!("{who:<10} GET {path:<28} {status}{to}");
    }

    // bob から見て、「他人のノート」と「無いノート」の答えを比べる
    let other = app.get("/notes/1", Some(&bob)).await;
    let missing = app.get("/notes/999", Some(&bob)).await;
    let (s1, s2) = (other.status(), missing.status());
    let (b1, b2) = (body_text(other).await, body_text(missing).await);
    println!("--- bob から見た2つの 404 ---");
    println!("状態コード: {s1} / {s2}");
    println!("本文の長さ: {} / {} バイト", b1.len(), b2.len());
    println!("本文がまったく同じ: {}", b1 == b2);
}
