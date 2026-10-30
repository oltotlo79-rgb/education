//! 第10巻 UNIT 03 練習問題。
//! 実行: cargo test --test u03_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, test_app};

const PASSWORD: &str = "correct horse battery";

/// 問1: ログインしていない人が、おかしな ID を開く
#[tokio::test]
async fn q1_guest_with_a_bad_id() {
    let app = test_app().await;
    for path in ["/notes/abc", "/notes/999"] {
        let response = app.get(path, None).await;
        let to = response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        println!("問1 guest GET {path} → {} → {to}", response.status());
    }
}

/// 問2: 同じノートを、いろいろな書き方の ID で開く
#[tokio::test]
async fn q2_other_spellings_of_one() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    for path in [
        "/notes/1",
        "/notes/01",
        "/notes/+1",
        "/notes/%31",
        "/notes/1.0",
        "/notes/%201",
    ] {
        let response = app.get(path, Some(&alice)).await;
        let status = response.status();
        let html = body_text(response).await;
        let edit = html
            .split("href=\"")
            .find(|s| s.contains("/edit\""))
            .map(|s| s.split('"').next().unwrap().to_string())
            .unwrap_or_else(|| "-".to_string());
        println!("問2 GET {path:<12} → {status}（編集の入り口: {edit}）");
    }
}

/// 問3: find_own_note を通らない操作（完了・削除）を、bob が alice のノートに送る
#[tokio::test]
async fn q3_bob_posts_to_alice_note() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    for path in ["/notes/1/toggle", "/notes/1/delete"] {
        let status = app.post_form(path, "", Some(&bob)).await.status();
        println!("問3 bob POST {path} → {status}");
    }
    let response = app.get("/notes/1", Some(&alice)).await;
    let status = response.status();
    let html = body_text(response).await;
    println!(
        "問3 alice GET /notes/1 → {status}（完了の印: {}）",
        html.contains("<span class=\"badge\">完了</span>")
    );
}
