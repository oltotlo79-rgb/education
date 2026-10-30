//! 第11巻 UNIT 03 問2: 守りに関わるヘッダーが、レスポンスにあるかを確かめる。
//! 実行: cargo test --test u03_q2 -- --nocapture

mod common;

use common::test_app;

const NAMES: [&str; 6] = [
    "x-content-type-options",
    "x-frame-options",
    "content-security-policy",
    "referrer-policy",
    "strict-transport-security",
    "cache-control",
];

#[tokio::test]
async fn security_headers() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for (path, cookie) in [("/login", None), ("/notes", Some(alice.as_str()))] {
        let response = app.get(path, cookie).await;
        println!("--- GET {path}（{}）", response.status());
        for name in NAMES {
            let value = response.headers().get(name);
            println!(
                "{name:<26} {}",
                value.map_or("無い".to_string(), |v| v.to_str().unwrap().to_string())
            );
        }
    }
}
