//! ノートのタイトルを調べる小さなライブラリ（品質コマンドの練習用）。

/// タイトルに使える最大の文字数。
pub const MAX_TITLE_CHARS: usize = 80;

/// タイトルが 1〜80 文字なら `true` を返す。
///
/// 前後の空白は数えない。文字数は [`str::chars`] で数えるので、
/// 日本語の「あ」も英字の「a」も同じ1文字になる。
///
/// # 例
///
/// ```
/// use quality_check::is_valid_title;
///
/// assert!(is_valid_title("買い物リスト"));
/// assert!(!is_valid_title("   "));
/// ```
pub fn is_valid_title(title: &str) -> bool {
    let count = title.trim().chars().count();
    (1..=MAX_TITLE_CHARS).contains(&count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_japanese_title() {
        assert!(is_valid_title("買い物リスト"));
    }

    #[test]
    fn rejects_blank_title() {
        assert!(!is_valid_title("   "));
    }

    #[test]
    fn accepts_exactly_80_chars() {
        let title = "あ".repeat(80);
        assert!(is_valid_title(&title), "80 文字は OK のはず");
    }

    #[test]
    fn rejects_81_chars() {
        let title = "あ".repeat(81);
        assert!(!is_valid_title(&title));
    }
}
