//! アプリ全体で共有する状態（AppState）。
//!
//! ルーターに `.app_context(state)` で登録しておくと、どのページの処理からでも
//! `app_state(cx)` で借りられる。保存先は `Arc<dyn ... + Send + Sync>` で持つので、
//! テストで別の実装に差し替えることもできる。

use std::sync::Arc;

use topcoat::context::{Cx, app_context};

use crate::db::Db;
use crate::repository::{
    LibsqlNoteRepository, LibsqlSessionRepository, LibsqlUserRepository, NoteRepository,
    SessionRepository, UserRepository,
};

/// アプリ全体で共有する状態。clone は Arc の参照を増やすだけなので安い
#[derive(Clone)]
pub struct AppState {
    /// DB そのもの（GET /health の確認に使う）
    pub db: Db,
    pub users: Arc<dyn UserRepository + Send + Sync>,
    pub notes: Arc<dyn NoteRepository + Send + Sync>,
    pub sessions: Arc<dyn SessionRepository + Send + Sync>,
}

impl AppState {
    /// libSQL 版のリポジトリを使う状態を作る（本番・開発・テストで共通）
    pub fn new(db: Db) -> AppState {
        AppState {
            users: Arc::new(LibsqlUserRepository::new(db.clone())),
            notes: Arc::new(LibsqlNoteRepository::new(db.clone())),
            sessions: Arc::new(LibsqlSessionRepository::new(db.clone())),
            db,
        }
    }
}

/// リクエストの処理の中から AppState を借りる
pub fn app_state(cx: &Cx) -> &AppState {
    app_context::<AppState>(cx)
}
