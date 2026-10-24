// lib の外からは、パッケージの名前（rustnote::）から読み込む
use rustnote::domain::NoteTitle;

fn main() {
    let title = NoteTitle::parse("牛乳を買う").expect("1〜80 文字です");
    println!("{title}");
}
