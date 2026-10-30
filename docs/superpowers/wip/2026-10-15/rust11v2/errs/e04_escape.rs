//! 第11巻 UNIT 04 エラー1: 正規表現の \ を、Rust の文字列の中で1つだけ書いた。
//! 実行: cargo test --test e04_escape

use proptest::prelude::*;
use rustnote::domain::NoteTitle;

proptest! {
    #[test]
    fn parse_never_panics(input in "\PC{0,100}") {
        let _ = NoteTitle::parse(&input);
    }
}
