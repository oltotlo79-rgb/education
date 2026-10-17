use std::fmt;

const MAX_TITLE_CHARS: usize = 80;

#[derive(Debug, PartialEq)]
enum TitleError {
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

// 検証に通ったタイトルだけを入れる型
#[derive(Debug)]
struct NoteTitle(String);

impl NoteTitle {
    // 1〜80 文字に収まっていれば NoteTitle を作る。そうでなければ理由を返す
    fn parse(raw: &str) -> Result<Self, TitleError> {
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

    fn as_str(&self) -> &str {
        &self.0
    }
}

// フォームから届いた「新しいノート」を検証済みの形で持つ
struct NewNote {
    title: NoteTitle,
    body: String,
}

impl NewNote {
    fn from_form(title: &str, body: &str) -> Result<Self, TitleError> {
        // タイトルが不正なら、? でその Err をそのまま返す
        let title = NoteTitle::parse(title)?;
        Ok(Self {
            title,
            body: body.trim().to_string(),
        })
    }
}

fn main() {
    let long = "長".repeat(81);
    let forms = [
        ("  牛乳を買う ", "低脂肪のもの"),
        ("", "本文だけ"),
        (long.as_str(), ""),
    ];
    for (title, body) in forms {
        match NewNote::from_form(title, body) {
            Ok(note) => println!(
                "作成: 「{}」（本文 {} 文字）",
                note.title.as_str(),
                note.body.chars().count()
            ),
            Err(e) => println!("入力エラー: {e}"),
        }
    }
}
