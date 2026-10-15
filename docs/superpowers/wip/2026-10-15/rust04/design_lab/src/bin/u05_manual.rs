use std::error::Error;
use std::fmt;

// タイトルが不正だった理由
#[derive(Debug, PartialEq, Eq)]
enum TitleError {
    Empty,
    TooLong { len: usize, max: usize },
}

// 利用者に見せる文章（{} で表示したとき）
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { len, max } => {
                write!(f, "タイトルは{max}文字以内にしてください（現在{len}文字）")
            }
        }
    }
}

// 「これはエラーを表す型です」という印。中身は既定の実装で足りる
impl Error for TitleError {}

fn parse_title(input: &str) -> Result<String, TitleError> {
    let trimmed = input.trim();
    let len = trimmed.chars().count();
    if len == 0 {
        return Err(TitleError::Empty);
    }
    if len > 80 {
        return Err(TitleError::TooLong { len, max: 80 });
    }
    Ok(trimmed.to_string())
}

fn main() {
    for input in ["買い物", "", &"あ".repeat(81)] {
        match parse_title(input) {
            Ok(title) => println!("OK: {title}"),
            Err(e) => {
                println!("利用者向け(Display): {e}");
                println!("開発者向け(Debug):   {e:?}");
            }
        }
    }

    // Error を実装したので、Box<dyn Error> にも入れられる
    let boxed: Box<dyn Error> = Box::new(TitleError::Empty);
    println!("Box<dyn Error>: {boxed}");
}
