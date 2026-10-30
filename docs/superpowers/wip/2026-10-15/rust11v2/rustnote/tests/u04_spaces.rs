//! 第11巻 UNIT 04: 入力の作り方を、間違いが出やすい形に寄せる。
//! 空白（半角・全角・タブ）と「a」だけで、0〜85 文字の文字列を作る。
//!
//! 実行: cargo test --test u04_spaces

use proptest::prelude::*;

/// 決まり: 前後の空白を除いた文字数が 1〜80 なら合格
fn rule(input: &str) -> bool {
    (1..=80).contains(&input.trim().chars().count())
}

/// わざと間違えた版: 前後の空白を除き忘れた
fn forgot_trim(input: &str) -> bool {
    (1..=80).contains(&input.chars().count())
}

proptest! {
    #[test]
    fn forgot_trim_with_spaces(input in "[ \t\u{3000}a]{0,85}") {
        prop_assert_eq!(forgot_trim(&input), rule(&input));
    }
}
