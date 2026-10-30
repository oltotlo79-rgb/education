//! PRG（Post/Redirect/Get）の流れを、1歩ずつ表示する。
//! 実行: cargo test --test flow -- --nocapture

mod common;

use common::{body_text, encode, location, test_app};
use topcoat::router::{header, response::Response};

const PASSWORD: &str = "correct horse battery";

// Set-Cookie のうち、flash= で始まるものをそのまま返す
fn flash_set_cookie(response: &Response) -> Option<String> {
    response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with("flash="))
        .map(str::to_string)
}

// 「はい」「いいえ」で表示する
fn yes_no(value: bool) -> &'static str {
    if value { "はい" } else { "いいえ" }
}

#[tokio::test]
async fn create_note_with_prg() {
    let app = test_app().await;
    let session = app.sign_up("alice@example.com", PASSWORD).await;

    // 1) 作成フォームを送る（POST）
    let form = format!("title={}&body=", encode("牛乳を買う"));
    let response = app.post_form("/notes", &form, Some(&session)).await;
    println!("1) POST /notes -> {}", response.status());
    println!("   Location: {}", location(&response));
    let set_cookie = flash_set_cookie(&response).expect("flash がありません");
    println!("   Set-Cookie: {set_cookie}");
    let url = location(&response).to_string();

    // 2) ブラウザは Location の URL を GET で開き直す（受け取った flash のクッキーも送る）
    let flash = set_cookie.split(';').next().unwrap();
    let cookies = format!("{session}; {flash}");
    let response = app.get(&url, Some(&cookies)).await;
    println!("2) GET {url} -> {}", response.status());
    if let Some(set_cookie) = flash_set_cookie(&response) {
        println!("   Set-Cookie: {set_cookie}");
    }
    let html = body_text(response).await;
    println!(
        "   「ノートを作成しました」が出た: {}",
        yes_no(html.contains("ノートを作成しました"))
    );

    // 3) 再読み込み（F5）。flash のクッキーは消えたので、セッションだけを送る
    let response = app.get(&url, Some(&session)).await;
    println!("3) GET {url} -> {}", response.status());
    let html = body_text(response).await;
    println!(
        "   「ノートを作成しました」が出た: {}",
        yes_no(html.contains("ノートを作成しました"))
    );
}

#[tokio::test]
async fn delete_note_with_prg() {
    let app = test_app().await;
    let session = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&session, "牛乳を買う", "").await;

    // 1) 削除のフォームを送る（POST）
    let path = format!("{url}/delete");
    let response = app.post_form(&path, "", Some(&session)).await;
    println!("1) POST {path} -> {}", response.status());
    println!("   Location: {}", location(&response));
    let set_cookie = flash_set_cookie(&response).expect("flash がありません");
    println!("   Set-Cookie: {set_cookie}");
    let next = location(&response).to_string();

    // 2) 移動先の一覧を開く。「ノートを削除しました」が出て、ノートは消えている
    let flash = set_cookie.split(';').next().unwrap();
    let cookies = format!("{session}; {flash}");
    let response = app.get(&next, Some(&cookies)).await;
    println!("2) GET {next} -> {}", response.status());
    let html = body_text(response).await;
    println!(
        "   「ノートを削除しました」が出た: {}",
        yes_no(html.contains("ノートを削除しました"))
    );
    println!(
        "   「牛乳を買う」が残っている: {}",
        yes_no(html.contains("牛乳を買う"))
    );
}
