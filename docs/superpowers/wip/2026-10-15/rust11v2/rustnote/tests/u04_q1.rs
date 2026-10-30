//! 第11巻 UNIT 04 問1: Email::parse の決まりを、property test で確かめる。
//! 実行: cargo test --test u04_q1 -- --nocapture

use proptest::prelude::*;
use rustnote::domain::Email;

/// メールアドレスらしい形（大文字・空白・全角も混ぜる）
fn email_like() -> impl Strategy<Value = String> {
    "[ ]{0,2}[a-zA-Z0-9.Ａ_]{0,10}@[a-zA-Z0-9.Ｂ-]{0,10}[ ]{0,2}"
}

proptest! {
    // 合格したアドレスを、もう一度 parse しても同じ
    #[test]
    fn parse_twice_is_the_same(input in email_like()) {
        if let Ok(email) = Email::parse(&input) {
            prop_assert_eq!(Email::parse(email.as_str()), Ok(email));
        }
    }

    // 合格したアドレスには、空白も大文字も無く、@ はちょうど1つ
    #[test]
    fn accepted_addresses_are_tidy(input in email_like()) {
        if let Ok(email) = Email::parse(&input) {
            let text = email.as_str();
            prop_assert!(!text.contains(char::is_whitespace));
            prop_assert_eq!(text, text.to_lowercase());
            prop_assert_eq!(text.matches('@').count(), 1);
        }
    }
}

// どのくらいの割合が合格するかも見ておく（合格が少なすぎると、上の確かめが空回りする）
#[test]
fn how_many_are_accepted() {
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    let mut runner = TestRunner::deterministic();
    let strategy = email_like();
    let mut accepted = 0;
    for _ in 0..1000 {
        let input = strategy.new_tree(&mut runner).unwrap().current();
        if Email::parse(&input).is_ok() {
            accepted += 1;
        }
    }
    println!("1000 個のうち、合格は {accepted} 個");
}
