//! 第8巻 UNIT 03: いろいろな入力で新規登録して、返事と、画面のエラーと、users 表に入った値を確かめる。
//! 実行: cargo test --test u03_signup -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, encode, test_app};

/// 画面の、入力欄の下のエラー（<p class="field-error">）を全部
fn field_errors(html: &str) -> Vec<String> {
    html.split("<p class=\"field-error\">")
        .skip(1)
        .map(|t| t.split("</p>").next().unwrap().to_string())
        .collect()
}

/// メールの入力欄に残っている値
fn email_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"email\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_else(|| "（なし）".to_string())
}

/// users 表の email を、id の順に
async fn emails(app: &TestApp) -> Vec<String> {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    found
}

#[tokio::test]
async fn signup_cases() {
    let app = test_app().await;
    let good = "correct horse battery";
    let cases = [
        ("alice@example.com", good, good),
        ("  Alice@Example.COM  ", good, good),
        ("alice", good, good),
        ("alice@example", good, good),
        ("bob@example.com", "short", "short"),
        ("bob@example.com", good, "correct horse batterY"),
        ("bob", "short", "other"),
        ("Bob@Example.com", good, good),
    ];
    for (n, (email, password, confirm)) in cases.into_iter().enumerate() {
        let form = format!(
            "email={}&password={}&password_confirm={}",
            encode(email),
            encode(password),
            encode(confirm)
        );
        let response = app.post_form("/signup", &form, None).await;
        let status = response.status();
        let html = body_text(response).await;
        println!("{}. email={email:?} → {status}", n + 1);
        if !status.is_redirection() {
            println!("   エラー: {:?}", field_errors(&html));
            println!("   メールの入力欄: {:?}", email_value(&html));
        }
    }
    println!("users 表の email: {:?}", emails(&app).await);
}
