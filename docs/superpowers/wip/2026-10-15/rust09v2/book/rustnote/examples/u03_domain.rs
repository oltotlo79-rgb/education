//! RustNote のドメインの型に、いろいろな入力を渡して、合格・不合格を確かめる。
//! 実行: cargo run -q --example u03_domain

use rustnote::domain::{Email, NoteId, NoteTitle, Password};

// 長い文字列は、先頭の 5 文字と文字数だけを表示する
fn short(text: &str) -> String {
    let count = text.chars().count();
    if count <= 10 {
        format!("{text:?}")
    } else {
        let head: String = text.chars().take(5).collect();
        format!("\"{head}…\"（{count} 文字）")
    }
}

fn main() {
    println!("--- NoteTitle::parse（前後の空白を除いて 1〜80 文字）");
    let eighty = "あ".repeat(80);
    let eighty_one = "あ".repeat(81);
    let titles = [
        ("牛乳を買う", "牛乳を買う"),
        ("前後に空白", "  買い物  "),
        ("空", ""),
        ("空白3つ", "   "),
        ("「あ」80個", eighty.as_str()),
        ("「あ」81個", eighty_one.as_str()),
    ];
    for (label, input) in titles {
        match NoteTitle::parse(input) {
            Ok(title) => println!("{label}: OK {}", short(title.as_str())),
            Err(error) => println!("{label}: NG {error}"),
        }
    }

    println!("--- Email::parse（小文字にそろえる）");
    for input in ["  Alice@Example.COM ", "alice", "alice@example"] {
        match Email::parse(input) {
            Ok(email) => println!("{input:?}: OK {:?}", email.as_str()),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }

    println!("--- NoteId（URL の /notes/{{id}} の部分）");
    for input in ["42", "0", "abc"] {
        match input.parse::<NoteId>() {
            Ok(id) => println!("{input:?}: OK {}", id.get()),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }

    println!("--- Password::parse（8〜128 文字。{{:?}} でも中身は出ない）");
    for input in ["short", "correct horse battery"] {
        match Password::parse(input) {
            Ok(password) => println!("{input:?}: OK {password:?}"),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }
}
