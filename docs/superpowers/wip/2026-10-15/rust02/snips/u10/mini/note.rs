// ノート1件と、そのタイトルの検証をまとめたモジュール
use std::fmt;

pub const MAX_TITLE_CHARS: usize = 80;

// ノートの ID。中身の i64 は外からも読めるように pub にする
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoteId(pub i64);

// タイトルの検証で起こりうる失敗
#[derive(Debug, PartialEq)]
pub enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { max, actual } => {
                write!(
                    f,
                    "タイトルは{max}文字以内にしてください（現在{actual}文字）"
                )
            }
        }
    }
}

// 検証に通ったタイトル。中身の String には pub を付けないので、
// モジュールの外では parse を通らないと作れない
#[derive(Debug, Clone)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn parse(raw: &str) -> Result<Self, TitleError> {
        let title = raw.trim();
        let count = title.chars().count();
        if count == 0 {
            return Err(TitleError::Empty);
        }
        if count > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: count,
            });
        }
        Ok(Self(title.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ノート1件
#[derive(Debug)]
pub struct Note {
    pub id: NoteId,
    pub title: NoteTitle,
    pub done: bool,
}
