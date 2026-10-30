//! 第13巻 UNIT 04 問1: NewNote::parse を ? で書いたら、どう変わるか。
//! 実行: cargo run -q --example u04_q1

use rustnote::domain::{NewNote, NoteBody, NoteTitle};

/// ? で書いた版（最初の失敗で止まる）
fn parse_with_question(title: &str, body: &str) -> Result<NewNote, String> {
    let title = NoteTitle::parse(title).map_err(|error| error.to_string())?;
    let body = NoteBody::parse(body).map_err(|error| error.to_string())?;
    Ok(NewNote { title, body })
}

fn main() {
    let long_body = "x".repeat(10_001);
    println!(
        "? で書いた版: {:?}",
        parse_with_question("", &long_body).map(|_| ())
    );

    let errors = NewNote::parse("", &long_body).unwrap_err();
    println!(
        "教材の版（タイトル）: {:?}",
        errors.title.map(|error| error.to_string())
    );
    println!(
        "教材の版（本文）: {:?}",
        errors.body.map(|error| error.to_string())
    );
}
