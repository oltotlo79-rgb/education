//! 寄り道: ページの処理の中で panic が起きると、どんな返事になるか。
//! 実行: cargo test --test u04_panic -- --nocapture

mod common;

use std::sync::Arc;

use async_trait::async_trait;
use common::{TestApp, body_text, test_assets, test_db};
use rustnote::{
    domain::{NewNote, Note, NoteId, NotePage, NoteQuery, UserId},
    error::RepoError,
    repository::NoteRepository,
    routes,
    state::AppState,
};

/// 「呼ばれないはず」のメソッドに unreachable! を書いた保存先
struct Unreachable;

#[async_trait]
impl NoteRepository for Unreachable {
    async fn insert(&self, _: UserId, _: &NewNote) -> Result<Note, RepoError> {
        unreachable!("insert は呼ばれないはず")
    }
    async fn find(&self, _: UserId, _: NoteId) -> Result<Option<Note>, RepoError> {
        unreachable!("find は呼ばれないはず")
    }
    async fn search(&self, _: UserId, _: &NoteQuery) -> Result<NotePage, RepoError> {
        unreachable!("search は呼ばれないはず")
    }
    async fn update(&self, _: UserId, _: NoteId, _: &NewNote, _: i64) -> Result<Note, RepoError> {
        unreachable!("update は呼ばれないはず")
    }
    async fn toggle_done(&self, _: UserId, _: NoteId) -> Result<Note, RepoError> {
        unreachable!("toggle_done は呼ばれないはず")
    }
    async fn delete(&self, _: UserId, _: NoteId) -> Result<(), RepoError> {
        unreachable!("delete は呼ばれないはず")
    }
}

#[tokio::test]
async fn panic_in_a_page_becomes_plain_500() {
    let db = test_db().await;
    let mut state = AppState::new(db.db.clone());
    state.notes = Arc::new(Unreachable);
    let app = TestApp {
        router: routes::build_router(state, test_assets()),
        db,
    };

    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    // 一覧のページは search を呼ぶので、unreachable! に当たる
    let response = app.get("/notes", Some(&alice)).await;
    println!("状態コード: {}", response.status());
    println!("本文: {:?}", body_text(response).await);
}
