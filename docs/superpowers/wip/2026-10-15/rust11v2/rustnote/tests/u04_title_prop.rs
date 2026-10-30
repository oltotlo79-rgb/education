//! 第11巻 UNIT 04: NoteTitle::parse の決まりを、でたらめな文字列で、たくさん確かめる（property test）。
//!
//! 実行: cargo test --test u04_title_prop -- --nocapture

use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::TestRunner;
use rustnote::domain::NoteTitle;

/// 決まり: 前後の空白を除いた文字数が 1〜80 なら合格
fn rule(input: &str) -> bool {
    (1..=80).contains(&input.trim().chars().count())
}

// 1. proptest が作る入力を、少しだけのぞいてみる（種を決めた TestRunner なので、毎回同じ）
#[test]
fn look_at_inputs() {
    let mut runner = TestRunner::deterministic();
    let strategy = "\\PC{0,100}";
    for _ in 0..5 {
        let input = strategy.new_tree(&mut runner).unwrap().current();
        println!(
            "{} 文字: {:?}…",
            input.chars().count(),
            input.chars().take(12).collect::<String>()
        );
    }
}

proptest! {
    // 2. どんな文字列でも、合格・不合格が、決まりと同じになる
    #[test]
    fn parse_follows_the_rule(input in "\\PC{0,100}") {
        prop_assert_eq!(NoteTitle::parse(&input).is_ok(), rule(&input));
    }

    // 3. 合格した題を、もう一度 parse しても、同じ題になる
    #[test]
    fn parse_twice_is_the_same(input in any::<String>()) {
        if let Ok(title) = NoteTitle::parse(&input) {
            prop_assert_eq!(NoteTitle::parse(title.as_str()), Ok(title));
        }
    }
}
