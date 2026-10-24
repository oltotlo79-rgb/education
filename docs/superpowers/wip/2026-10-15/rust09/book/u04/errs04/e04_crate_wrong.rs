// lib.rs の中と同じつもりで、crate:: からドメインの型を読み込もうとした
use crate::domain::NoteTitle;

fn main() {
    let title = NoteTitle::parse("牛乳を買う").expect("1〜80 文字です");
    println!("{title}");
}
