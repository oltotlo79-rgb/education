//! 第8巻 UNIT 03 練習問題。
//! 実行: cargo test --test u03_q -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, encode, test_app};

const PASSWORD: &str = "correct horse battery";

/// 画面の、入力欄の下のエラー・フォームの上のお知らせ
fn messages(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    for start in ["<p class=\"field-error\">", "<p class=\"form-notice\">"] {
        for t in html.split(start).skip(1) {
            found.push(t.split("</p>").next().unwrap().to_string());
        }
    }
    found
}

async fn signup(app: &TestApp, email: &str, password: &str) -> String {
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode(email),
        encode(password),
        encode(password)
    );
    let response = app.post_form("/signup", &form, None).await;
    let status = response.status();
    let html = body_text(response).await;
    format!("{status} {:?}", messages(&html))
}

async fn login(app: &TestApp, email: &str, password: &str) -> String {
    let form = format!("email={}&password={}", encode(email), encode(password));
    let response = app.post_form("/login", &form, None).await;
    let status = response.status();
    let html = body_text(response).await;
    format!("{status} {:?}", messages(&html))
}

/// 問1: 登録の画面とログインの画面は、alice がいることを、それぞれどう伝えるか
#[tokio::test]
async fn q1_signup_tells_more_than_login() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    println!(
        "問1 登録（alice）: {}",
        signup(&app, "alice@example.com", PASSWORD).await
    );
    println!(
        "問1 登録（carol）: {}",
        signup(&app, "carol@example.com", PASSWORD).await
    );
    println!(
        "問1 ログイン（alice・違うパスワード）: {}",
        login(&app, "alice@example.com", "wrong password").await
    );
    println!(
        "問1 ログイン（dave・登録していない）: {}",
        login(&app, "dave@example.com", "wrong password").await
    );
}

/// 問2: 全角の英字で書いたメールアドレス
#[tokio::test]
async fn q2_full_width_letters() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    println!(
        "問2 登録（ＡＬＩＣＥ@example.com）: {}",
        signup(&app, "ＡＬＩＣＥ@example.com", PASSWORD).await
    );
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    println!("問2 users 表の email: {found:?}");
}

/// 問3: パスワードの前後の空白
#[tokio::test]
async fn q3_spaces_in_password() {
    let app = test_app().await;
    let spaced = "  correct horse battery  ";
    println!(
        "問3 登録（前後に空白）: {}",
        signup(&app, "alice@example.com", spaced).await
    );
    println!(
        "問3 ログイン（空白なし）: {}",
        login(&app, "alice@example.com", PASSWORD).await
    );
    println!(
        "問3 ログイン（前後に空白）: {}",
        login(&app, "alice@example.com", spaced).await
    );
}
