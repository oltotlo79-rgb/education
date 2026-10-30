//! 第13巻 UNIT 04: 検査済みの型の parse と、エラーの分類を、いろいろな入力で確かめる。
//! 実行: cargo run -q --example u04_types

use rustnote::domain::{Email, NewNote, NoteBody, NoteId, NoteQuery, NoteTitle, Password};
use rustnote::error::{AppError, RepoError};

fn main() {
    println!("--- NoteTitle::parse");
    for input in ["  買い物  ", "", "   ", "\t買い物\t", "\0"] {
        println!("{input:?} → {:?}", NoteTitle::parse(input));
    }
    for (name, input) in [
        ("a×80", "a".repeat(80)),
        ("a×81", "a".repeat(81)),
        ("😀×80", "😀".repeat(80)),
    ] {
        println!(
            "{name} → {:?}",
            NoteTitle::parse(&input).map(|t| t.as_str().chars().count())
        );
    }

    println!("--- NoteBody::parse");
    println!("{:?}", NoteBody::parse("1行目\r\n2行目"));
    println!(
        "10001 文字 → {:?}",
        NoteBody::parse(&"x".repeat(10_001)).map(|_| ())
    );

    println!("--- NewNote::parse（両方まちがい）");
    println!("{:?}", NewNote::parse("", &"x".repeat(10_001)));

    println!("--- Email::parse");
    for input in [
        "  Alice@Example.COM ",
        "alice",
        "@example.com",
        "alice@example",
        "a@b@c.com",
        "ａｌｉｃｅ@example.com",
    ] {
        println!("{input:?} → {:?}", Email::parse(input));
    }

    println!("--- Password::parse");
    for input in [
        "short",
        "correct horse battery",
        " 空白も 文字 ",
        "あいうえおかきく",
    ] {
        println!("{input:?} → {:?}", Password::parse(input));
    }

    println!("--- NoteId（URL の {{id}}）");
    for input in ["42", "0", "-3", "abc", "01", " 1", "9223372036854775808"] {
        println!("{input:?} → {:?}", input.parse::<NoteId>());
    }

    println!("--- NoteQuery::new");
    let long = "あ".repeat(150);
    for (keyword, page) in [
        (Some("  rust "), Some(3)),
        (Some("   "), Some(0)),
        (Some(long.as_str()), None),
    ] {
        let query = NoteQuery::new(keyword, page);
        let shown = query.keyword.as_ref().map(|k| k.chars().count());
        println!(
            "キーワードの文字数 {shown:?}・page {}・offset {}",
            query.page,
            query.offset()
        );
    }

    println!("--- AppError の分類（状態コード・画面に出す文）");
    let errors = [
        AppError::from(NoteTitle::parse("").unwrap_err()),
        AppError::from(RepoError::NotFound),
        AppError::from(RepoError::VersionMismatch),
        AppError::from(RepoError::UniqueViolation(
            "UNIQUE constraint failed: users.email".to_string(),
        )),
        AppError::EmailTaken,
        AppError::internal("disk full"),
    ];
    for error in &errors {
        println!(
            "{:?} → {}・「{}」",
            error_name(error),
            error.status_code(),
            error.public_message()
        );
    }
}

/// 種類の名前だけを取り出す（中身は表示しない）
fn error_name(error: &AppError) -> &'static str {
    match error {
        AppError::InvalidTitle(_) => "InvalidTitle",
        AppError::InvalidBody(_) => "InvalidBody",
        AppError::InvalidEmail(_) => "InvalidEmail",
        AppError::InvalidPassword(_) => "InvalidPassword",
        AppError::Unauthorized => "Unauthorized",
        AppError::NotFound => "NotFound",
        AppError::EmailTaken => "EmailTaken",
        AppError::Conflict => "Conflict",
        AppError::Internal(_) => "Internal",
    }
}
