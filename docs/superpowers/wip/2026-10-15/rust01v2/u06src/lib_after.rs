/// ノートの題の文字数を数える
pub fn title_length(title: &str) -> usize {
    title.chars().count()
}

/// 題が空（空白だけ）かどうか
pub fn is_blank(title: &str) -> bool {
    title.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_title() {
        assert_eq!(title_length("買い物"), 3);
    }

    #[test]
    fn blank_title() {
        assert!(is_blank("   "));
        assert!(!is_blank("メモ"));
    }
}
