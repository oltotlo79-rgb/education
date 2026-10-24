//! AppState のノートの保存先を、わざと失敗する物に差し替えて、
//! 「DB が壊れたとき」の画面（受入条件の障害の枠）を確かめる。
//! 実行: cargo test --test swap -- --nocapture

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
use topcoat::router::StatusCode;

/// どの操作も失敗する、ノートの保存先（テスト用）
struct BrokenNotes;

fn broken() -> RepoError {
    RepoError::InvalidData("ディスクが壊れました（テストのための失敗）".to_string())
}

#[async_trait]
impl NoteRepository for BrokenNotes {
    async fn insert(&self, _: UserId, _: &NewNote) -> Result<Note, RepoError> {
        Err(broken())
    }
    async fn find(&self, _: UserId, _: NoteId) -> Result<Option<Note>, RepoError> {
        Err(broken())
    }
    async fn search(&self, _: UserId, _: &NoteQuery) -> Result<NotePage, RepoError> {
        Err(broken())
    }
    async fn update(&self, _: UserId, _: NoteId, _: &NewNote, _: i64) -> Result<Note, RepoError> {
        Err(broken())
    }
    async fn toggle_done(&self, _: UserId, _: NoteId) -> Result<Note, RepoError> {
        Err(broken())
    }
    async fn delete(&self, _: UserId, _: NoteId) -> Result<(), RepoError> {
        Err(broken())
    }
}

// 前提: ノートの保存先が壊れている（ユーザーとセッションの保存先は動いている）
// 操作: ログインした人が、ノートの一覧を開く
// 結果: 500 で「時間をおいてもう一度お試しください」とだけ出す。壊れた理由は画面に出さない
#[tokio::test]
async fn notes_page_is_500_when_note_storage_is_broken() {
    let db = test_db().await;
    let mut state = AppState::new(db.db.clone());
    state.notes = Arc::new(BrokenNotes); // ここだけを差し替える
    let app = TestApp {
        router: routes::build_router(state, test_assets()),
        db,
    };

    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let response = app.get("/notes", Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let html = body_text(response).await;
    assert!(html.contains("時間をおいてもう一度お試しください"));
    assert!(
        !html.contains("ディスクが壊れました"),
        "壊れた理由が画面に出ています"
    );
}
