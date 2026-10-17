// 最初の1文字を返す。空文字列なら None
fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

fn main() {
    for text in ["Rust", "日本語", ""] {
        match first_char(text) {
            Some(c) => println!("{text:?} の最初の文字: {c}"),
            None => println!("{text:?} は空です"),
        }
    }
    println!("代わりの値: {}", first_char("").unwrap_or('?'));
}
