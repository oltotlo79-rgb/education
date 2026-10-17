//! RustNote のドメイン型を集めたライブラリです。
//!
//! - [`NoteTitle`] … 1〜80 文字に検証済みのタイトル
//! - [`NoteStatus`] … ノートの状態（下書き・公開・アーカイブ）と、その移り変わり

pub mod status;
pub mod title;

pub use status::{Action, NoteStatus, TransitionError};
pub use title::{MAX_TITLE_CHARS, NoteTitle, TitleError};
