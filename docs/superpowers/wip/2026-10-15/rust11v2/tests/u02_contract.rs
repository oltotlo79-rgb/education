//! 第11巻 UNIT 02: NoteRepository の約束を、本物の DB（一時フォルダの libSQL ファイル）で1つずつ確かめる。
//! 実行: cargo test --test u02_contract -- --nocapture

mod common;

use rustnote::{
    domain::{Email, NewNote, Note, NoteId},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

/// ノートの大事な所だけを、1行の文字列にする
fn brief(result: Result<Note, RepoError>) -> String {
    match result {
        Ok(note) => format!(
            "Ok（id={} version={} done={} title={:?}）",
            note.id, note.version, note.done, note.title
        ),
        Err(error) => format!("Err({error:?})"),
    }
}

fn new_note(title: &str) -> NewNote {
    NewNote::parse(title, "").expect("テスト用のノート")
}

#[tokio::test]
async fn note_repository_contract() {
    // Arrange: 空の DB と、2人のユーザー
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

    // Act と Assert: 約束を1つずつ
    let created = notes.insert(alice.id, &new_note("初版")).await.unwrap();
    let id = created.id;
    println!(
        "insert（alice）             → {}",
        brief(Ok(created.clone()))
    );
    println!(
        "   作成と更新の日時が同じ   → {}",
        created.created_at == created.updated_at
    );

    let found = notes.find(alice.id, id).await.unwrap();
    println!(
        "find（alice）               → 見つかる: {}",
        found.is_some()
    );
    let found = notes.find(bob.id, id).await.unwrap();
    println!(
        "find（bob）                 → 見つかる: {}",
        found.is_some()
    );

    let result = notes.update(alice.id, id, &new_note("第2版"), 1).await;
    println!("update（alice, version 1）  → {}", brief(result));
    let result = notes.update(alice.id, id, &new_note("古い画面"), 1).await;
    println!("update（alice, version 1）  → {}", brief(result));
    let missing = NoteId::new(99).unwrap();
    let result = notes.update(alice.id, missing, &new_note("無い"), 1).await;
    println!("update（alice, id 99）      → {}", brief(result));
    let result = notes.update(bob.id, id, &new_note("乗っ取り"), 2).await;
    println!("update（bob, version 2）    → {}", brief(result));

    let result = notes.toggle_done(alice.id, id).await;
    println!("toggle_done（alice）        → {}", brief(result));

    println!(
        "delete（bob）               → {:?}",
        notes.delete(bob.id, id).await
    );
    println!(
        "delete（alice）             → {:?}",
        notes.delete(alice.id, id).await
    );
    println!(
        "delete（alice, もう一度）   → {:?}",
        notes.delete(alice.id, id).await
    );
}
