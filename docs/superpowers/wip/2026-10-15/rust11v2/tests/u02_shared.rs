//! 第11巻 UNIT 02 のエラー2: 2つのテストが、同じ DB ファイルを使い回したとき。
//! 実行: cargo test --test u02_shared

use rustnote::{
    db::{self, Db},
    domain::Email,
    repository::{LibsqlUserRepository, UserRepository},
};

/// どのテストも、決まった1つのファイルを開いてしまう（一時フォルダを作らない）
async fn shared_db() -> Db {
    let path = std::env::temp_dir().join("rustnote-shared-test.db");
    let db = Db::open_local(path).await.expect("DB を開けません");
    db::migrate(&db).await.expect("マイグレーションに失敗");
    db
}

#[tokio::test]
async fn sign_up_alice() {
    let users = LibsqlUserRepository::new(shared_db().await);
    let email = Email::parse("alice@example.com").unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap();
}

#[tokio::test]
async fn alice_can_be_found() {
    let users = LibsqlUserRepository::new(shared_db().await);
    let email = Email::parse("alice@example.com").unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap();
    assert!(users.find_by_email(&email).await.unwrap().is_some());
}
