//! 第8巻 UNIT 04: ログインのたびに、セッションのトークンが新しくなり、前のトークンが使えなくなるかを確かめる。
//! 実行: cargo test --test u04_renew -- --nocapture

mod common;

use common::{TestApp, encode, session_cookie, test_app};

/// sessions 表の行の数
async fn sessions(app: &TestApp) -> i64 {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT COUNT(*) FROM sessions", ())
        .await
        .unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

/// クッキーの値の先頭6文字（全部は表示しない）
fn short(cookie: &str) -> String {
    let value = cookie.trim_start_matches("__Host-session=");
    format!("{}…", &value[..6])
}

#[tokio::test]
async fn token_is_renewed() {
    let app = test_app().await;
    let first = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    println!(
        "1. 登録したときのトークン A: {}（sessions: {} 行）",
        short(&first),
        sessions(&app).await
    );

    // A を持ったまま、もう一度ログインする
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode("correct horse battery")
    );
    let response = app.post_form("/login", &form, Some(&first)).await;
    let second = session_cookie(&response).unwrap();
    println!(
        "2. ログインしたあとのトークン B: {}（sessions: {} 行）",
        short(&second),
        sessions(&app).await
    );
    println!("   A と B は同じ: {}", first == second);

    for (name, cookie) in [("A", &first), ("B", &second)] {
        let response = app.get("/notes", Some(cookie)).await;
        let to = response
            .headers()
            .get("location")
            .map(|v| format!(" → {}", v.to_str().unwrap()))
            .unwrap_or_default();
        println!(
            "3. トークン {name} で GET /notes → {}{to}",
            response.status()
        );
    }
}
