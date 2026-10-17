// tests/ フォルダのファイルは、ライブラリを「外から」使うテスト（結合テスト）
use note_domain::{Action, NoteStatus, NoteTitle, TitleError};

#[test]
fn create_publish_and_fail_to_publish_archived() {
    let title = NoteTitle::parse("週報を書く").expect("正しいタイトル");
    assert_eq!(title.as_str(), "週報を書く");

    let status = NoteStatus::Draft.apply(Action::Publish).unwrap();
    let status = status.apply(Action::Archive).unwrap();

    let err = status.apply(Action::Publish).unwrap_err();
    assert_eq!(err.to_string(), "Archived の状態では Publish できません");
}

#[test]
fn error_message_for_too_long_title() {
    let err = NoteTitle::parse(&"x".repeat(100)).unwrap_err();
    assert_eq!(err, TitleError::TooLong(100));
    assert_eq!(
        err.to_string(),
        "タイトルは80文字以内にしてください（現在100文字）"
    );
}
