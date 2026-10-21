use std::time::Instant;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

fn main() {
    let start = Instant::now();
    // Argon2id（既定の設定）でハッシュ化する。塩（salt）は毎回ランダムに作られる
    let phc = Argon2::default()
        .hash_password(b"correct horse battery staple")
        .expect("ハッシュ化に失敗しました")
        .to_string();
    println!("ハッシュ化: {:.2} 秒", start.elapsed().as_secs_f64());
    println!("{phc}");

    let start = Instant::now();
    let parsed = PasswordHash::new(&phc).expect("PHC 文字列を読めません");
    let ok = Argon2::default()
        .verify_password(b"correct horse battery staple", &parsed)
        .is_ok();
    println!("照合: {ok}（{:.2} 秒）", start.elapsed().as_secs_f64());
}
