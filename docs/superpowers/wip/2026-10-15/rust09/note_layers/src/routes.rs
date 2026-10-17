//! ルート: 外から来た文字列を domain の型に変え、
//! repository の「約束（trait）」を通して保存する

use crate::domain::NoteTitle;
use crate::repository::NoteRepository;

/// POST /notes の代わり。返事を文字列で返す
pub fn create_note(repo: &mut impl NoteRepository, input: &str) -> String {
    match NoteTitle::parse(input) {
        Ok(title) => {
            let id = repo.insert(title);
            format!("303 → /notes/{id}")
        }
        Err(message) => format!("400 {message}"),
    }
}
