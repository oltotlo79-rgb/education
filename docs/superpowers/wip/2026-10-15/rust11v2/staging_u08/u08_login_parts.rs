//! 第11巻 UNIT 08: ログインの1回を、部分ごとに測る（どこが遅いかを、直す前に確かめる）。
//! 実行: cargo test --test u08_login_parts -- --nocapture
//!       cargo test --release --test u08_login_parts -- --nocapture

mod common;

use std::time::{Duration, Instant};

use common::{encode, test_app};
use rustnote::auth::verify_password;
use rustnote::domain::Email;
use rustnote::repository::{LibsqlUserRepository, UserRepository};

/// 5回測って、いちばん短い時間（ほかの仕事の影響が少ない回）を返す
async fn fastest<F, Fut>(mut f: F) -> Duration
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut best = Duration::MAX;
    for _ in 0..5 {
        let started = Instant::now();
        f().await;
        best = best.min(started.elapsed());
    }
    best
}

#[tokio::test]
async fn where_does_login_spend_time() {
    let app = test_app().await;
    app.sign_up("alice@example.com", "correct horse battery").await;
    let users = LibsqlUserRepository::new(app.db.db.clone());
    let email = Email::parse("alice@example.com").unwrap();
    let hash = users.find_by_email(&email).await.unwrap().unwrap().password_hash;

    let lookup = fastest(|| async {
        users.find_by_email(&email).await.unwrap();
    })
    .await;
    let verify = fastest(|| async {
        verify_password("correct horse battery", &hash).await.unwrap();
    })
    .await;
    let form = format!("email={}&password={}", encode("alice@example.com"), encode("correct horse battery"));
    let whole = fastest(|| async {
        app.post_form("/login", &form, None).await;
    })
    .await;

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    println!("--- {profile} ビルド（5回のうち、いちばん短い時間）");
    println!("DB でユーザーを探す : {lookup:>10.2?}");
    println!("パスワードの照合   : {verify:>10.2?}");
    println!("POST /login 全体   : {whole:>10.2?}");
    println!("照合が全体に占める割合: {:.0}%", verify.as_secs_f64() / whole.as_secs_f64() * 100.0);
}
