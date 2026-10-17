// 別の package（domain）の公開された型を使う
use domain::NoteTitle;

fn main() {
    for input in ["牛乳を買う", "  "] {
        match NoteTitle::parse(input) {
            Ok(title) => println!("OK: {}", title.as_str()),
            Err(e) => println!("NG: {e}"),
        }
    }
}
