// タイトルの文字数を数える（この版にはバグがある）
fn title_length(title: &str) -> usize {
    title.len()
}

// 1 文字以上 80 文字以下なら true（正しいタイトル）
fn is_valid_title(title: &str) -> bool {
    let length = title_length(title);
    (1..=80).contains(&length)
}

fn main() {
    for title in ["Rust", "牛乳を買う", ""] {
        let length = title_length(title);
        let valid = is_valid_title(title);
        println!("{title:?}: 文字数 {length}, 正しい? {valid}");
    }
}

// ここから下は cargo test のときだけ使われる「テスト」
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_characters() {
        assert_eq!(title_length("牛乳を買う"), 5);
    }

    #[test]
    fn empty_title_is_invalid() {
        assert!(!is_valid_title(""));
    }

    #[test]
    fn eighty_characters_is_valid() {
        // 「あ」を 80 個並べた文字列を作る
        let title = "あ".repeat(80);
        assert!(is_valid_title(&title));
    }

    #[test]
    fn eighty_one_characters_is_invalid() {
        let title = "あ".repeat(81);
        assert!(!is_valid_title(&title));
    }
}
