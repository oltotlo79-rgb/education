//! 第13巻 UNIT 06 エラー1: 別のスレッドで動かす処理に、借りたパスワードをそのまま渡した。
//! 実行: cargo run -q --example e06_borrow

use argon2::{Argon2, password_hash::PasswordHasher};
use rustnote::domain::Password;

async fn hash(password: &Password) -> String {
    tokio::task::spawn_blocking(|| {
        Argon2::default()
            .hash_password(password.as_str().as_bytes())
            .map(|hash| hash.to_string())
    })
    .await
    .unwrap()
    .unwrap()
}

#[tokio::main]
async fn main() {
    let password = Password::parse("correct horse battery").unwrap();
    println!("{}", hash(&password).await);
}
