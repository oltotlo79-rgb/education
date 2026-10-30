//! 第8巻 UNIT 10: 事故の練習。「DB の中身が漏れたかもしれない」とわかったときに、
//! 漏れた DB の値で、攻撃する人が何をできるかと、全員のセッションを、
//! サーバーの側で一度に終わらせられるかを確かめる。
//!
//! 実行: cargo test --test u10_incident -- --nocapture

mod common;

use common::{TestApp, encode, test_app};
use topcoat::session::Token;

const PASSWORD: &str = "correct horse battery";

/// 先頭の4バイトを、16 進数で表示する
fn head(bytes: &[u8]) -> String {
    bytes[..4]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// sessions 表の、最初の行の token_hash（32 バイト）
async fn leaked_token_hash(app: &TestApp) -> [u8; 32] {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT token_hash FROM sessions LIMIT 1", ())
        .await
        .unwrap();
    let bytes: Vec<u8> = rows.next().await.unwrap().unwrap().get(0).unwrap();
    bytes.try_into().unwrap()
}

/// users 表の、最初の行の password_hash（PHC 文字列）
async fn leaked_password_hash(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT password_hash FROM users LIMIT 1", ())
        .await
        .unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

#[tokio::test]
async fn leaked_database() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;

    // 1. 漏れた DB の値を、そのまま使ってみる
    let hash = leaked_token_hash(&app).await;
    let password_hash = leaked_password_hash(&app).await;
    println!("1. 漏れた token_hash: {}…（32 バイト）", head(&hash));
    println!("   漏れた password_hash: {}…", &password_hash[..30]);
    // token_hash を、本物のクッキーと同じ形（32 バイトの base64）にして送る
    let forged = Token::new(hash);
    let cookie = format!("__Host-session={}", forged.encode());
    let status = app.get("/notes", Some(&cookie)).await.status();
    println!("   token_hash をクッキーの形にして GET /notes → {status}");
    println!(
        "   （サーバーが DB で探すのは、その SHA-256: {}…）",
        head(forged.hash().as_slice())
    );
    // PHC 文字列を、そのままパスワードとして送る
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode(&password_hash)
    );
    let status = app.post_form("/login", &form, None).await.status();
    println!("   password_hash をパスワードにしてログイン → {status}");

    // 2. 全員のセッションを終わらせる
    let conn = app.db.db.connect().await.unwrap();
    let deleted = conn.execute("DELETE FROM sessions", ()).await.unwrap();
    println!("2. DELETE FROM sessions → {deleted} 行");
    for (who, cookie) in [("alice", &alice), ("bob", &bob)] {
        let status = app.get("/notes", Some(cookie)).await.status();
        println!("   {who} のクッキーで GET /notes → {status}");
    }
}
