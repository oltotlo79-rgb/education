use std::fmt;

const MAX_TITLE_CHARS: usize = 80;

// タイトルの検証で起こりうる失敗を enum で列挙する
#[derive(Debug, PartialEq)]
enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

// 利用者に見せるメッセージを決める（{} で表示できるようになる）
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

fn check_title(raw: &str) -> Result<String, TitleError> {
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
    Ok(title.to_string())
}

fn main() {
    let long = "あ".repeat(81);
    for raw in ["  牛乳を買う ", "   ", long.as_str()] {
        match check_title(raw) {
            Ok(title) => println!("OK: {title}"),
            Err(e) => println!("NG: {e}  /  {e:?}"),
        }
    }
    // PartialEq を derive したので、結果を == で比べられる（テストで便利）
    println!("{}", check_title("") == Err(TitleError::Empty));
}
