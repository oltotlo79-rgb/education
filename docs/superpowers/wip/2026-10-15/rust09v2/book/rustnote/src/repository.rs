//! リポジトリ（データのしまい方・取り出し方を引き受ける部品）。
//!
//! それぞれ「約束（trait）」と「libSQL で実装したもの」をセットで置いている。
//! - notes.rs    … `NoteRepository` / `LibsqlNoteRepository`
//! - users.rs    … `UserRepository` / `LibsqlUserRepository`
//! - sessions.rs … `SessionRepository` / `LibsqlSessionRepository`
//!
//! ページの処理（routes）は trait だけを知っていて、AppState に入った
//! `Arc<dyn NoteRepository + Send + Sync>` を通して使う。
//!
//! DB の操作は待ち時間があるので、メソッドは async fn にしている。
//! 普通の async fn を持つ trait は dyn（実行時に実装を選ぶ形）で使えないため、
//! `#[async_trait]` を付けて dyn で使える形に変換している。

mod notes;
mod sessions;
mod users;

use std::time::{SystemTime, UNIX_EPOCH};

pub use notes::{LibsqlNoteRepository, NoteRepository};
pub use sessions::{LibsqlSessionRepository, SessionRepository};
pub use users::{LibsqlUserRepository, UserRecord, UserRepository};

use crate::error::RepoError;

/// SystemTime を「1970-01-01 からの秒数」（sessions.expires_at の形）に変える
pub fn unix_seconds(time: SystemTime) -> i64 {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    i64::try_from(seconds).unwrap_or(i64::MAX)
}

/// DB から読んだ値の変換に失敗したら InvalidData にする
fn invalid_data(what: &str, error: impl std::fmt::Display) -> RepoError {
    RepoError::InvalidData(format!("{what}: {error}"))
}
