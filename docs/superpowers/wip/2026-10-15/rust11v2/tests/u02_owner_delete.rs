//! 第11巻 UNIT 02 のエラー1の直し方: 失敗したときに「実際の値」が出る形で確かめる。
//! 実行: cargo test --test u02_owner_delete

mod common;

use rustnote::{
    domain::{Email, NewNote},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

#[tokio::test]
async fn other_user_cannot_delete() {
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
    let bob = users
        .create(&Email::parse("bob@example.com").unwrap(), "$argon2id$dummy")
        .await
        .unwrap();
    let note = notes
        .insert(alice.id, &NewNote::parse("買い物", "").unwrap())
        .await
        .unwrap();

    let result = notes.delete(bob.id, note.id).await;
    // 2つ目の引数に、失敗したときに出す文と、実際の値を書いておく
    assert!(
        matches!(result, Err(RepoError::NotFound)),
        "bob の削除は NotFound のはずが、{result:?} だった"
    );
    let still_there = notes.find(alice.id, note.id).await.unwrap();
    assert!(still_there.is_some(), "alice のノートが消えている");
}
