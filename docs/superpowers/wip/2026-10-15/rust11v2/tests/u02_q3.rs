//! 問3の解答: ユーザーを消すと、その人のノートとセッションはどうなるかを確かめる。
//! 実行: cargo test --test u02_q3 -- --nocapture

mod common;

use std::time::{Duration, SystemTime};

use rustnote::{
    db::Db,
    domain::{Email, NewNote},
    repository::{
        LibsqlNoteRepository, LibsqlSessionRepository, LibsqlUserRepository, NoteRepository,
        SessionRepository, UserRepository,
    },
};
use topcoat::session::TokenHash;

/// 表ごとの行の数を、1行の文字列にする
async fn counts(db: &Db) -> String {
    let conn = db.connect().await.expect("DB に接続できません");
    let mut parts = Vec::new();
    for table in ["users", "notes", "sessions"] {
        let mut rows = conn
            .query(&format!("SELECT COUNT(*) FROM {table}"), ())
            .await
            .expect("数えられません");
        let row = rows
            .next()
            .await
            .expect("行を読めません")
            .expect("行がない");
        let count: i64 = row.get(0).expect("COUNT(*)");
        parts.push(format!("{table}={count}"));
    }
    parts.join(" ")
}

#[tokio::test]
async fn deleting_a_user_cascades() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let expires = SystemTime::now() + Duration::from_secs(60);
    for (n, email) in ["alice@example.com", "bob@example.com"]
        .into_iter()
        .enumerate()
    {
        let user = users
            .create(&Email::parse(email).unwrap(), "$argon2id$dummy")
            .await
            .unwrap();
        for title in ["1件目", "2件目"] {
            notes
                .insert(user.id, &NewNote::parse(title, "").unwrap())
                .await
                .unwrap();
        }
        let hash = TokenHash::new([n as u8; 32]);
        sessions.create(&hash, user.id, expires).await.unwrap();
    }
    println!("消す前: {}", counts(&test.db).await);

    // RustNote には「退会」の機能がまだ無いので、SQL で直接 alice を消す
    let conn = test.db.connect().await.unwrap();
    let deleted = conn
        .execute("DELETE FROM users WHERE email = 'alice@example.com'", ())
        .await
        .unwrap();
    println!("users から {deleted} 行消した");
    println!("消した後: {}", counts(&test.db).await);
}
