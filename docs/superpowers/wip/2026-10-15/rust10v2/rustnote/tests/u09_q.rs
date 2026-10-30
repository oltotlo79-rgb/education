//! 第10巻 UNIT 09 練習問題。
//! 実行: cargo test --test u09_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, encode, location, test_app};

const PASSWORD: &str = "correct horse battery";

/// 画面のお知らせの文。無ければ "（なし）"
fn banner(html: &str) -> String {
    html.split("role=\"status\">")
        .nth(1)
        .and_then(|t| t.split("</p>").next())
        .unwrap_or("（なし）")
        .to_string()
}

/// 問1: flash のクッキーを、手で作って送る
#[tokio::test]
async fn q1_handmade_cookie() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for value in [
        "%3Cscript%3Ealert(1)%3C%2Fscript%3E",
        "note_deleted",
        "NOTE_DELETED",
    ] {
        let html = body_text(
            app.get("/notes", Some(&format!("{alice}; flash={value}")))
                .await,
        )
        .await;
        println!("問1 flash={value} → お知らせ「{}」", banner(&html));
    }
}

/// 問2: タブ A で作成した直後に、タブ B で一覧を開く
#[tokio::test]
async fn q2_two_tabs() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let form = format!("title={}&body=", encode("牛乳を買う"));
    let response = app.post_form("/notes", &form, Some(&alice)).await;
    let to = location(&response).to_string();
    let flash = "flash=note_created";
    println!(
        "問2 タブ A: POST /notes → {}（移動先 {to}）",
        response.status()
    );
    // ブラウザのクッキーは、タブの間で共有される
    let response = app.get("/notes", Some(&format!("{alice}; {flash}"))).await;
    let removed = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .any(|v| v.to_str().unwrap().starts_with("flash=;"));
    let html = body_text(response).await;
    println!("問2 タブ B: GET /notes → お知らせ「{}」", banner(&html));
    let cookie = if removed {
        alice.clone()
    } else {
        format!("{alice}; {flash}")
    };
    let html = body_text(app.get(&to, Some(&cookie)).await).await;
    println!("問2 タブ A: GET {to} → お知らせ「{}」", banner(&html));
}
