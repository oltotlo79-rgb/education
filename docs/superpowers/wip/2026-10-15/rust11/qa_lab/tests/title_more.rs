use qa_lab::title::{NoteTitle, TitleError};

/// 合格するはずの入力を確かめる。
/// #[track_caller] を付けると、失敗したとき「この関数を呼んだ行」が表示される
#[track_caller]
fn assert_ok(input: &str, expected: &str) {
    match NoteTitle::parse(input) {
        Ok(title) => assert_eq!(title.as_str(), expected, "入力 {input:?}"),
        Err(error) => panic!(
            "{}文字の入力が {error:?} で不合格になりました",
            input.chars().count()
        ),
    }
}

/// 不合格になるはずの入力を確かめる
#[track_caller]
fn assert_err(input: &str, expected: TitleError) {
    assert_eq!(NoteTitle::parse(input), Err(expected));
}

#[test]
fn helpers_point_to_the_caller() {
    assert_ok("a", "a");
    assert_ok(&"あ".repeat(80), &"あ".repeat(80));
    assert_err("", TitleError::Empty);
    assert_err(&"あ".repeat(81), TitleError::TooLong { len: 81, max: 80 });
}

/// 結果を「Ok(80文字)」のような短い文字列にする（失敗の一覧を読みやすくするため）
fn describe(result: &Result<String, TitleError>) -> String {
    match result {
        Ok(title) => format!("Ok({}文字)", title.chars().count()),
        Err(error) => format!("Err({error:?})"),
    }
}

#[test]
fn report_all_failures_at_once() {
    let cases = [
        ("空文字", String::new(), Err(TitleError::Empty)),
        ("1文字", "a".to_string(), Ok("a".to_string())),
        ("80文字ちょうど", "あ".repeat(80), Ok("あ".repeat(80))),
        (
            "81文字",
            "あ".repeat(81),
            Err(TitleError::TooLong { len: 81, max: 80 }),
        ),
        ("絵文字は1文字", "👍".to_string(), Ok("👍".to_string())),
    ];

    // 失敗をすぐに panic させず、全部集めてから最後に 1 回だけ報告する
    let mut failures = Vec::new();
    for (name, input, expected) in cases {
        let actual = NoteTitle::parse(&input).map(|t| t.as_str().to_string());
        if actual != expected {
            failures.push(format!(
                "  - {name}: 期待 {} / 実際 {}",
                describe(&expected),
                describe(&actual)
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{}件のケースが失敗しました:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
