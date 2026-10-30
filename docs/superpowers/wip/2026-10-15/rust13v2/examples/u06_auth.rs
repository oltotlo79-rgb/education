//! 第13巻 UNIT 06: パスワードのハッシュ（Argon2id）を作って照合し、時間を測る。
//! 実行: cargo run -q --release --example u06_auth

use std::error::Error as _;
use std::time::Instant;

use rustnote::auth::{hash_password, verify_password};
use rustnote::domain::Password;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let password = Password::parse("correct horse battery")?;
    let started = Instant::now();
    let first = hash_password(&password).await?;
    let hash_ms = started.elapsed().as_millis();
    let second = hash_password(&password).await?;

    // PHC 文字列: $方式$版$設定$塩$ハッシュ
    let parts: Vec<&str> = first.split('$').collect();
    let second_parts: Vec<&str> = second.split('$').collect();
    println!("方式 {}・版 {}・設定 {}", parts[1], parts[2], parts[3]);
    println!(
        "塩 {} 文字・ハッシュ {} 文字",
        parts[4].len(),
        parts[5].len()
    );
    println!("同じパスワードの2回目と同じ文字列か: {}", first == second);
    println!("塩は同じか: {}", parts[4] == second_parts[4]);
    println!("ハッシュを作る時間: {hash_ms} ミリ秒");

    for input in [
        "correct horse battery",
        "correct horse batterY",
        " correct horse battery",
    ] {
        let started = Instant::now();
        let matches = verify_password(input, &first).await?;
        println!(
            "{input:?} → {matches}（{} ミリ秒）",
            started.elapsed().as_millis()
        );
    }

    match verify_password("x", "not-a-phc").await {
        Ok(matches) => println!("壊れた PHC → Ok({matches})"),
        Err(error) => println!(
            "壊れた PHC → 画面の文「{}」・原因「{}」",
            error.public_message(),
            error.source().map(|s| s.to_string()).unwrap_or_default()
        ),
    }
    Ok(())
}
