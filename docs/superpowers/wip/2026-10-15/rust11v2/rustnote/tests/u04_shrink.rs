//! 第11巻 UNIT 04: わざと間違えた parse を、property test で確かめる。
//! 失敗した入力を、proptest が「もっと小さい入力」に縮めていく（shrink）様子を見る。
//!
//! 実行: cargo test --test u04_shrink -- --test-threads=1

use proptest::prelude::*;

/// 決まり: 前後の空白を除いた文字数が 1〜80 なら合格
fn rule(input: &str) -> bool {
    (1..=80).contains(&input.trim().chars().count())
}

/// わざと間違えた版1: 前後の空白を除き忘れた
fn forgot_trim(input: &str) -> bool {
    (1..=80).contains(&input.chars().count())
}

/// わざと間違えた版2: 文字数ではなく、バイト数で数えた
fn counted_bytes(input: &str) -> bool {
    (1..=80).contains(&input.trim().len())
}

proptest! {
    #[test]
    fn a_forgot_trim(input in "\\PC{0,100}") {
        prop_assert_eq!(forgot_trim(&input), rule(&input));
    }

    #[test]
    fn b_counted_bytes(input in "\\PC{0,100}") {
        prop_assert_eq!(counted_bytes(&input), rule(&input));
    }
}
