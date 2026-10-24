//! 問3の解答: ノートの保存先が壊れているとき、ノートの作成（POST /notes）も 500 になり、
//! 入力したタイトルも、壊れた理由も、画面に出さないことを確かめる。
//! 実行: cargo test --test u04_q3 -- --nocapture

mod common;

use std::sync::{Arc, Mutex};

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

/// どの操作も失敗し、呼ばれたメソッドの名前を記録する保存先（テスト用）
#[derive(Default)]
struct RecordingBrokenNotes {
    calls: Mutex<Vec<&'static str>>,
}

impl RecordingBrokenNotes {
    /// 呼ばれた名前を記録して、失敗を返す
    fn fail(&self, name: &'static str) -> RepoError {
        self.calls.lock().expect("記録を読めません").push(name);
        RepoError::InvalidData(format!("{name} に失敗（テストのための失敗）"))
    }
}

#[async_trait]
impl NoteRepository for RecordingBrokenNotes {
    async fn insert(&self, _: UserId, _: &NewNote) -> Result<Note, RepoError> {
        Err(self.fail("insert"))
    }
    async fn find(&self, _: UserId, _: NoteId) -> Result<Option<Note>, RepoError> {
        Err(self.fail("find"))
    }
    async fn search(&self, _: UserId, _: &NoteQuery) -> Result<NotePage, RepoError> {
        Err(self.fail("search"))
    }
    async fn update(&self, _: UserId, _: NoteId, _: &NewNote, _: i64) -> Result<Note, RepoError> {
        Err(self.fail("update"))
    }
    async fn toggle_done(&self, _: UserId, _: NoteId) -> Result<Note, RepoError> {
        Err(self.fail("toggle_done"))
    }
    async fn delete(&self, _: UserId, _: NoteId) -> Result<(), RepoError> {
        Err(self.fail("delete"))
    }
}

#[tokio::test]
async fn creating_a_note_is_500_when_note_storage_is_broken() {
    let db = test_db().await;
    let notes = Arc::new(RecordingBrokenNotes::default());
    let mut state = AppState::new(db.db.clone());
    // 記録を後で読むので、Arc を複製して入れる（中身は同じ1つの保存先）
    state.notes = notes.clone();
    let app = TestApp {
        router: routes::build_router(state, test_assets()),
        db,
    };

    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    // タイトルは「秘密のメモ」を URL エンコードしたもの
    let form = "title=%E7%A7%98%E5%AF%86%E3%81%AE%E3%83%A1%E3%83%A2&body=";
    let response = app.post_form("/notes", form, Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let html = body_text(response).await;
    assert!(html.contains("時間をおいてもう一度お試しください"));
    assert!(
        !html.contains("テストのための失敗"),
        "壊れた理由が画面に出ています"
    );
    assert!(
        !html.contains("秘密のメモ"),
        "入力したタイトルが画面に出ています"
    );

    let calls = notes.calls.lock().expect("記録を読めません").clone();
    println!("呼ばれたメソッド: {calls:?}");
    assert_eq!(calls, ["insert"], "insert だけが呼ばれるはずです");
}
