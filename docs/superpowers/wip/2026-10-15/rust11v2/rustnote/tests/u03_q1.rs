//! 第11巻 UNIT 03 問1: ログインしていない人が GET /notes を開いたときの、4つの部分を確かめる。
//! 実行: cargo test --test u03_q1 -- --nocapture

mod common;

use common::{body_text, location, test_app};
use topcoat::router::{StatusCode, header};

#[tokio::test]
async fn guest_opens_notes() {
    let app = test_app().await;
    let response = app.get("/notes", None).await;

    let status = response.status();
    let to = location(&response).to_string();
    let cookies: Vec<String> = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|value| value.to_str().unwrap().to_string())
        .collect();
    let html = body_text(response).await;
    println!("状態コード: {status}");
    println!("Location: {to}");
    println!("Set-Cookie: {cookies:?}");
    println!("本文: {} バイト", html.len());

    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(to, "/login");
    assert_eq!(
        cookies,
        ["flash=login_required; HttpOnly; SameSite=Lax; Path=/; Max-Age=300"]
    );

    // 移動先のログイン画面で、お知らせが出る
    let html = body_text(app.get("/login", Some("flash=login_required")).await).await;
    println!(
        "移動先に「ログインしてください」: {}",
        html.contains("ログインしてください")
    );
}
