use thiserror::Error;

pub const MAX_TITLE_CHARS: usize = 80;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{MAX_TITLE_CHARS}文字以内にしてください（現在{0}文字）")]
    TooLong(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
        let t = input.trim();
        let len = t.chars().count();
        if len == 0 {
            return Err(TitleError::Empty);
        }
        if len > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong(len));
        }
        Ok(NoteTitle(t.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
