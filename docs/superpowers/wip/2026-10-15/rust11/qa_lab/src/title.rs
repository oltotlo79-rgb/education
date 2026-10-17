//! RustNote の src/domain.rs から NoteTitle の部分だけを写したもの。

use std::fmt;

use thiserror::Error;

/// タイトルの最大文字数（chars().count() で数える）
pub const MAX_TITLE_CHARS: usize = 80;

/// タイトルが不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}

/// 検査済みのタイトル（前後の空白を除いて 1〜80 文字）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub const MAX_CHARS: usize = MAX_TITLE_CHARS;

    /// 文字列を検査し、合格したときだけ NoteTitle を返す
    pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
        let trimmed = input.trim();
        // 文字数は chars().count() で数える（len() はバイト数なので日本語で狂う）
        let len = trimmed.chars().count();
        if len == 0 {
            return Err(TitleError::Empty);
        }
        if len > Self::MAX_CHARS {
            return Err(TitleError::TooLong {
                len,
                max: Self::MAX_CHARS,
            });
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NoteTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Arrange（準備）→ Act（実行）→ Assert（確認）の 3 段で書く
    #[test]
    fn trims_spaces_around_title() {
        // Arrange: 前後に空白のある入力
        let input = "  買い物  ";
        // Act: 検査する
        let result = NoteTitle::parse(input);
        // Assert: 空白が取れたタイトルになっている
        assert_eq!(result.unwrap().as_str(), "買い物");
    }

    // 境界値: 0・1・80・81 文字の 4 点を確かめる
    #[test]
    fn boundary_values() {
        assert_eq!(NoteTitle::parse(""), Err(TitleError::Empty));
        assert!(NoteTitle::parse("あ").is_ok());
        assert!(NoteTitle::parse(&"あ".repeat(80)).is_ok());
        assert_eq!(
            NoteTitle::parse(&"あ".repeat(81)),
            Err(TitleError::TooLong { len: 81, max: 80 })
        );
    }
}
