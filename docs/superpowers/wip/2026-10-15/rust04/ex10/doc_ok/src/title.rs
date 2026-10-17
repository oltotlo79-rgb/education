use thiserror::Error;

/// タイトルの最大文字数です。
pub const MAX_TITLE_CHARS: usize = 80;

/// タイトルが正しくなかった理由です。
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TitleError {
    /// 空白を取り除くと空になった。
    #[error("タイトルを入力してください")]
    Empty,
    /// [`MAX_TITLE_CHARS`] 文字を超えた（中身は実際の文字数）。
    #[error("タイトルは{MAX_TITLE_CHARS}文字以内にしてください（現在{0}文字）")]
    TooLong(usize),
}

/// 前後の空白を除いて 1〜80 文字であることが保証されたタイトルです。
///
/// 作る方法は [`NoteTitle::parse`] だけです。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    /// 文字列を検査して `NoteTitle` を作ります。前後の空白は取り除きます。
    ///
    /// 文字数は `chars().count()`（Unicode の文字の数）で数えます。
    ///
    /// # Errors
    ///
    /// - 空白を除いて空なら [`TitleError::Empty`]
    /// - 80 文字を超えたら [`TitleError::TooLong`]
    ///
    /// # Examples
    ///
    /// ```
    /// use note_domain::{NoteTitle, TitleError};
    ///
    /// let title = NoteTitle::parse("  牛乳を買う ").unwrap();
    /// assert_eq!(title.as_str(), "牛乳を買う");
    ///
    /// assert_eq!(NoteTitle::parse("   "), Err(TitleError::Empty));
    /// ```
    pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
        let trimmed = input.trim();
        let len = trimmed.chars().count();
        if len == 0 {
            return Err(TitleError::Empty);
        }
        if len > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong(len));
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    /// タイトルの文字列を借ります。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// cargo test のときだけコンパイルされる
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_spaces() {
        let title = NoteTitle::parse("  牛乳を買う  ").unwrap();
        assert_eq!(title.as_str(), "牛乳を買う");
    }

    #[test]
    fn rejects_empty_and_spaces_only() {
        assert_eq!(NoteTitle::parse(""), Err(TitleError::Empty));
        assert_eq!(NoteTitle::parse(" \t "), Err(TitleError::Empty));
    }

    // 境界値: 80 文字ちょうどは OK、81 文字は NG
    #[test]
    fn boundary_80_and_81_chars() {
        let ok = "あ".repeat(MAX_TITLE_CHARS);
        let ng = "あ".repeat(MAX_TITLE_CHARS + 1);
        assert!(NoteTitle::parse(&ok).is_ok());
        assert_eq!(NoteTitle::parse(&ng), Err(TitleError::TooLong(81)));
    }

    // Result を返すテストでは、中で ? が使える
    #[test]
    fn single_char_is_ok() -> Result<(), TitleError> {
        let title = NoteTitle::parse("a")?;
        assert_eq!(title.as_str(), "a");
        Ok(())
    }

    // パニックすることを確かめるテスト
    #[test]
    #[should_panic(expected = "Empty")]
    fn unwrap_on_empty_panics() {
        NoteTitle::parse("").unwrap();
    }
}
