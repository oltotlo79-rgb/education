//! リポジトリの結合テスト: 一時フォルダの本物の libSQL ファイルに対して読み書きする。
//! 実行: cargo test --test repository

mod common;

use std::time::{Duration, SystemTime};

use rustnote::{
    db::{self, MIGRATIONS},
    domain::{Email, NewNote, NoteQuery, PER_PAGE, User},
    error::RepoError,
    repository::{
        LibsqlNoteRepository, LibsqlSessionRepository, LibsqlUserRepository, NoteRepository,
        SessionRepository, UserRepository,
    },
};
use topcoat::session::TokenHash;

/// テスト用のユーザーを登録する
async fn create_user(users: &LibsqlUserRepository, email: &str) -> User {
    let email = Email::parse(email).unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap()
}

fn new_note(title: &str, body: &str) -> NewNote {
    NewNote::parse(title, body).unwrap()
}

#[tokio::test]
async fn migrations_are_applied_only_once() {
    let test = common::test_db().await;
    // test_db() で1回適用済みなので、2回目は何も適用されない
    let applied = db::migrate(&test.db).await.unwrap();
    assert!(applied.is_empty());

    let conn = test.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT COUNT(*) FROM schema_migrations", ())
        .await
        .unwrap();
    let count = rows.next().await.unwrap().unwrap().get::<i64>(0).unwrap();
    assert_eq!(count, MIGRATIONS.len() as i64);
}

#[tokio::test]
async fn duplicate_email_is_a_unique_violation() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    create_user(&users, "alice@example.com").await;

    let email = Email::parse("ALICE@example.com").unwrap();
    let result = users.create(&email, "$argon2id$dummy").await;
    assert!(matches!(result, Err(RepoError::UniqueViolation(_))));

    let found = users.find_by_email(&email).await.unwrap().unwrap();
    assert_eq!(found.user.email.as_str(), "alice@example.com");
    assert_eq!(found.password_hash, "$argon2id$dummy");
}

#[tokio::test]
async fn notes_are_isolated_per_owner() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;
    let bob = create_user(&users, "bob@example.com").await;

    let note = notes
        .insert(alice.id, &new_note("買い物", "牛乳"))
        .await
        .unwrap();
    assert_eq!(note.title, "買い物");
    assert_eq!(note.version, 1);
    assert!(!note.done);

    // 持ち主なら見つかる。他人からは見えない
    assert!(notes.find(alice.id, note.id).await.unwrap().is_some());
    assert!(notes.find(bob.id, note.id).await.unwrap().is_none());

    // 他人は更新・切り替え・削除もできない（NotFound）
    let changed = new_note("乗っ取り", "");
    assert!(matches!(
        notes.update(bob.id, note.id, &changed, 1).await,
        Err(RepoError::NotFound)
    ));
    assert!(matches!(
        notes.toggle_done(bob.id, note.id).await,
        Err(RepoError::NotFound)
    ));
    assert!(matches!(
        notes.delete(bob.id, note.id).await,
        Err(RepoError::NotFound)
    ));
}

#[tokio::test]
async fn update_uses_optimistic_locking() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;
    let note = notes.insert(alice.id, &new_note("初版", "")).await.unwrap();

    // version 1 で更新 → 成功して version 2 になる
    let updated = notes
        .update(alice.id, note.id, &new_note("第2版", "本文"), 1)
        .await
        .unwrap();
    assert_eq!(updated.title, "第2版");
    assert_eq!(updated.version, 2);

    // 古い version 1 のまま更新しようとすると VersionMismatch
    let stale = notes
        .update(alice.id, note.id, &new_note("古い画面から", ""), 1)
        .await;
    assert!(matches!(stale, Err(RepoError::VersionMismatch)));

    // 中身は第2版のまま
    let current = notes.find(alice.id, note.id).await.unwrap().unwrap();
    assert_eq!(current.title, "第2版");
}

#[tokio::test]
async fn toggle_and_delete() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;
    let note = notes.insert(alice.id, &new_note("宿題", "")).await.unwrap();

    let toggled = notes.toggle_done(alice.id, note.id).await.unwrap();
    assert!(toggled.done);
    assert_eq!(toggled.version, 2);
    let toggled = notes.toggle_done(alice.id, note.id).await.unwrap();
    assert!(!toggled.done);

    notes.delete(alice.id, note.id).await.unwrap();
    assert!(notes.find(alice.id, note.id).await.unwrap().is_none());
    assert!(matches!(
        notes.delete(alice.id, note.id).await,
        Err(RepoError::NotFound)
    ));
}

#[tokio::test]
async fn search_filters_by_keyword_and_paginates() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;
    let bob = create_user(&users, "bob@example.com").await;

    // alice のノートを PER_PAGE + 1 件（21件）、bob のノートを1件
    for i in 1..=(PER_PAGE + 1) {
        notes
            .insert(alice.id, &new_note(&format!("メモ {i}"), ""))
            .await
            .unwrap();
    }
    notes
        .insert(alice.id, &new_note("100% 本気", "割引の_計算"))
        .await
        .unwrap();
    notes
        .insert(bob.id, &new_note("bob のメモ", ""))
        .await
        .unwrap();

    // キーワード無し: 1ページ目は 20 件、全体は 22 件、2 ページ
    let page1 = notes
        .search(alice.id, &NoteQuery::new(None, None))
        .await
        .unwrap();
    assert_eq!(page1.notes.len(), PER_PAGE as usize);
    assert_eq!(page1.total_count, 22);
    assert_eq!(page1.total_pages(), 2);
    assert!(page1.notes.iter().all(|n| n.owner == alice.id));

    let page2 = notes
        .search(alice.id, &NoteQuery::new(None, Some(2)))
        .await
        .unwrap();
    assert_eq!(page2.notes.len(), 2);

    // キーワード検索（タイトルにも本文にも当たる）
    let found = notes
        .search(alice.id, &NoteQuery::new(Some("メモ 2"), None))
        .await
        .unwrap();
    // "メモ 2" と "メモ 20" と "メモ 21"
    assert_eq!(found.total_count, 3);

    // % や _ は「ただの文字」として検索される
    let percent = notes
        .search(alice.id, &NoteQuery::new(Some("100%"), None))
        .await
        .unwrap();
    assert_eq!(percent.total_count, 1);
    let underscore = notes
        .search(alice.id, &NoteQuery::new(Some("_計算"), None))
        .await
        .unwrap();
    assert_eq!(underscore.total_count, 1);
}

#[tokio::test]
async fn sessions_expire() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;

    let now = SystemTime::now();
    let hash = TokenHash::new([7; 32]);
    sessions
        .create(&hash, alice.id, now + Duration::from_secs(60))
        .await
        .unwrap();

    // 期限内なら見つかる
    let user = sessions.find_user(&hash, now).await.unwrap();
    assert_eq!(user.map(|u| u.id), Some(alice.id));

    // 期限を過ぎた時刻で探すと見つからない
    let later = now + Duration::from_secs(120);
    assert!(sessions.find_user(&hash, later).await.unwrap().is_none());

    // 期限切れの掃除
    assert_eq!(sessions.delete_expired(later).await.unwrap(), 1);
    assert!(sessions.find_user(&hash, now).await.unwrap().is_none());
}

#[tokio::test]
async fn deleting_a_session_logs_out() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let alice = create_user(&users, "alice@example.com").await;

    let now = SystemTime::now();
    let hash = TokenHash::new([9; 32]);
    sessions
        .create(&hash, alice.id, now + Duration::from_secs(60))
        .await
        .unwrap();
    sessions.delete(&hash).await.unwrap();
    assert!(sessions.find_user(&hash, now).await.unwrap().is_none());
}
