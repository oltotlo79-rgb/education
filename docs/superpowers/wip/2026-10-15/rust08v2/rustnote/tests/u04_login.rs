//! 第8巻 UNIT 04: いろいろな入力でログインして、返事と、画面の文と、2つの失敗の画面の違いを確かめる。
//! 実行: cargo test --test u04_login -- --nocapture --test-threads=1

mod common;

use common::{body_text, encode, session_cookie, test_app};

const PASSWORD: &str = "correct horse battery";

/// 画面のお知らせ（form-notice）の文
fn notice(html: &str) -> String {
    html.split("<p class=\"form-notice\">")
        .nth(1)
        .and_then(|t| t.split("</p>").next())
        .unwrap_or("（なし）")
        .to_string()
}

#[tokio::test]
async fn login_cases() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;

    let cases = [
        ("alice@example.com", PASSWORD),
        ("ALICE@example.com", PASSWORD),
        ("alice@example.com", "correct horse batterY"),
        ("dave@example.com", PASSWORD),
        ("alice", PASSWORD),
        ("", ""),
    ];
    let mut pages = Vec::new();
    for (n, (email, password)) in cases.into_iter().enumerate() {
        let form = format!("email={}&password={}", encode(email), encode(password));
        let response = app.post_form("/login", &form, None).await;
        let status = response.status();
        let cookie = session_cookie(&response).is_some();
        let html = body_text(response).await;
        println!("{}. email={email:?} → {status}", n + 1);
        if status.is_redirection() {
            println!("   新しいセッションのクッキー: {cookie}");
        } else {
            println!("   文: {}", notice(&html));
            // 入力欄に残る自分のメールを伏せて、画面どうしを比べられるようにする
            let masked = html.replace(&format!("value=\"{email}\""), "value=\"…\"");
            pages.push((n + 1, masked));
        }
    }
    println!("--- 失敗の画面どうしを比べる（入力欄のメールは伏せた）");
    let (first_n, first) = &pages[0];
    for (n, page) in &pages[1..] {
        println!("{first_n} と {n}: まったく同じ = {}", first == page);
    }
}
