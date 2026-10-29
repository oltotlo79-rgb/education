//! 問1の解答: 検索（LIKE）は、大文字と小文字を区別するかを確かめる。
//! 実行: cargo test --test u02_q1 -- --nocapture

mod common;

use rustnote::{
    domain::{Email, NewNote, NoteQuery},
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

#[tokio::test]
async fn like_and_letter_case() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();
    for (title, body) in [("Rust 入門", "SQLite の LIKE"), ("Äpfel kaufen", "")] {
        notes
            .insert(alice.id, &NewNote::parse(title, body).unwrap())
            .await
            .unwrap();
    }

    for keyword in [
        "Rust",
        "rust",
        "RUST",
        "ｒｕｓｔ",
        "sqlite",
        "Äpfel",
        "äpfel",
    ] {
        let page = notes
            .search(alice.id, &NoteQuery::new(Some(keyword), None))
            .await
            .unwrap();
        println!("{keyword:<8} → {} 件", page.total_count);
    }
}
