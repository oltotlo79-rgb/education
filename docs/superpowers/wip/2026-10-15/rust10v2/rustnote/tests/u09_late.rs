//! 第10巻 UNIT 09: 完了を切り替えたあと、詳細の画面と、そのあとに開いた一覧の画面に、お知らせが出るかを確かめる。
//! 実行: cargo test --test u09_late -- --nocapture

mod common;

use common::{body_text, test_app};

/// 画面のお知らせの文。無ければ "（なし）"
fn banner(html: &str) -> String {
    html.split("role=\"status\">")
        .nth(1)
        .and_then(|t| t.split("</p>").next())
        .unwrap_or("（なし）")
        .to_string()
}

#[tokio::test]
async fn which_page_shows_the_flash() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;

    let response = app.post_form("/notes/1/toggle", "", Some(&alice)).await;
    let mut flash: Option<String> = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap().split(';').next().unwrap().to_string())
        .find(|c| c.starts_with("flash="));
    println!(
        "1. POST /notes/1/toggle → {}（{}）",
        response.status(),
        flash.as_deref().unwrap_or("")
    );

    for (n, path) in [(2, "/notes/1"), (3, "/notes")] {
        let cookie = match &flash {
            Some(flash) => format!("{alice}; {flash}"),
            None => alice.clone(),
        };
        let response = app.get(path, Some(&cookie)).await;
        // 消す Set-Cookie が来たら、ブラウザと同じように、flash を忘れる
        let removed = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .any(|v| v.to_str().unwrap().starts_with("flash=;"));
        if removed {
            flash = None;
        }
        let html = body_text(response).await;
        println!(
            "{n}. GET {path}: お知らせ「{}」・flash を消した: {removed}",
            banner(&html)
        );
    }
}
