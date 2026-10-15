mod domain {
    use std::fmt;

    // 中身の String は private（pub を付けない）。外から直接作れない
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NoteTitle(String);

    // タイトルが不正だった理由
    #[derive(Debug, PartialEq, Eq)]
    pub enum TitleError {
        Empty,
        TooLong { len: usize },
    }

    impl NoteTitle {
        pub const MAX_CHARS: usize = 80;

        // 文字列を検査し、合格したときだけ NoteTitle を返す
        pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
            let trimmed = input.trim();
            // 文字数は chars().count() で数える（len() はバイト数）
            let len = trimmed.chars().count();
            if len == 0 {
                return Err(TitleError::Empty);
            }
            if len > Self::MAX_CHARS {
                return Err(TitleError::TooLong { len });
            }
            Ok(NoteTitle(trimmed.to_string()))
        }

        // 中身を読むだけの窓口（書き換える窓口は用意しない）
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    // {} で表示したときの見た目
    impl fmt::Display for NoteTitle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }
}

use domain::NoteTitle;

fn main() {
    let inputs = ["  買い物リスト  ", "", "   ", &"あ".repeat(81)];
    for input in inputs {
        match NoteTitle::parse(input) {
            Ok(title) => println!("OK: [{title}] ({}文字)", title.as_str().chars().count()),
            Err(e) => println!("NG: {e:?}"),
        }
    }
}
