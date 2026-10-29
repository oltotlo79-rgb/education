//! 問2の解答: セッションの期限ちょうどの時刻で、探す・掃除するとどうなるかを確かめる。
//! 実行: cargo test --test u02_q2 -- --nocapture

mod common;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rustnote::{
    domain::Email,
    repository::{
        LibsqlSessionRepository, LibsqlUserRepository, SessionRepository, UserRepository,
    },
};
use topcoat::session::TokenHash;

#[tokio::test]
async fn session_expiry_boundary() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();

    // 期限（T）を、決まった時刻にしておく（今の時刻に頼らない）
    let t = UNIX_EPOCH + Duration::from_secs(2_000_000_000);
    let second = Duration::from_secs(1);
    let hash = TokenHash::new([7; 32]);
    sessions.create(&hash, alice.id, t).await.unwrap();

    for (label, now) in [
        ("T - 1秒", t - second),
        ("T ちょうど", t),
        ("T + 1秒", t + second),
    ] {
        let found = sessions.find_user(&hash, now).await.unwrap();
        println!("find_user（{label}）→ 見つかる: {}", found.is_some());
    }
    let before: SystemTime = t - second;
    println!(
        "delete_expired（T - 1秒）→ {} 件消した",
        sessions.delete_expired(before).await.unwrap()
    );
    println!(
        "delete_expired（T ちょうど）→ {} 件消した",
        sessions.delete_expired(t).await.unwrap()
    );
}
