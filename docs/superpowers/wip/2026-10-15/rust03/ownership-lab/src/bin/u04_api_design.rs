// 入力は読むだけなので &str、結果は新しく作るので String
fn normalize_email(input: &str) -> String {
    input.trim().to_lowercase()
}

struct NoteTitle(String);

impl NoteTitle {
    // 検証に通ったら、所有する String を中に持った NoteTitle を返す
    fn parse(input: &str) -> Result<NoteTitle, String> {
        let trimmed = input.trim();
        let count = trimmed.chars().count();
        if count == 0 || count > 80 {
            return Err(format!("タイトルは1〜80文字です（今は{count}文字）"));
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    // 中の文字列を借りて見せる
    fn as_str(&self) -> &str {
        &self.0
    }
}

// 本文の先頭 max_chars 文字ぶんを「借りて」返す。新しい String は作らない
fn preview(body: &str, max_chars: usize) -> &str {
    match body.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &body[..byte_pos],
        None => body,
    }
}

// 完了フラグの一覧を借りて、true の数を数える
fn count_done(flags: &[bool]) -> usize {
    flags.iter().filter(|done| **done).count()
}

fn main() {
    let form_email = String::from("  Alice@Example.COM ");
    println!("[{}]", normalize_email(&form_email));

    for input in ["  買い物リスト  ", "   "] {
        match NoteTitle::parse(input) {
            Ok(title) => println!("OK: [{}]", title.as_str()),
            Err(message) => println!("NG: {message}"),
        }
    }

    let body = String::from("牛乳、卵、パン、バター、ヨーグルト");
    println!("{}…", preview(&body, 8));
    println!("{}", preview("短い", 8));

    let flags = vec![true, false, true, true];
    println!("完了 {} 件", count_done(&flags));
    println!("先頭2件のうち完了 {} 件", count_done(&flags[..2]));
}
