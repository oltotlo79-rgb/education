use qa_lab::title::{NoteTitle, TitleError};

#[test]
fn note_title_table() {
    let a80 = "あ".repeat(80);
    let a81 = "あ".repeat(81);
    // (ケースの名前, 入力, 期待する結果)。Ok の中身は「空白を除いたあとのタイトル」
    let cases: Vec<(&str, &str, Result<&str, TitleError>)> = vec![
        ("空文字", "", Err(TitleError::Empty)),
        ("半角空白だけ", "   ", Err(TitleError::Empty)),
        ("全角空白だけ", "\u{3000}\u{3000}", Err(TitleError::Empty)),
        ("タブと改行だけ", "\t\n", Err(TitleError::Empty)),
        ("1文字", "a", Ok("a")),
        ("前後の空白を除く", "  買い物  ", Ok("買い物")),
        ("80文字ちょうど", &a80, Ok(&a80)),
        (
            "81文字",
            &a81,
            Err(TitleError::TooLong { len: 81, max: 80 }),
        ),
        ("絵文字は1文字", "👍", Ok("👍")),
        ("結合文字は2文字と数える", "か\u{3099}", Ok("か\u{3099}")),
    ];

    for (name, input, expected) in cases {
        // Ok(NoteTitle) を Ok(&str) に、Err(&TitleError) を Err(TitleError) にそろえて比べる
        let actual = NoteTitle::parse(input);
        let actual = actual.as_ref().map(NoteTitle::as_str).map_err(|e| *e);
        assert_eq!(actual, expected, "ケース「{name}」: 入力 {input:?}");
    }
}
