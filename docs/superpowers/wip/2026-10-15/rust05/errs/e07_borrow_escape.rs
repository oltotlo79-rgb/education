use argon2::{Argon2, password_hash::PasswordHasher};

async fn hash_password(password: &str) -> String {
    // 借り物の password を、そのまま別スレッドへ持ち込もうとする
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .unwrap()
            .to_string()
    })
    .await
    .unwrap()
}

#[tokio::main]
async fn main() {
    println!("{}", hash_password("rust-is-fun").await);
}
