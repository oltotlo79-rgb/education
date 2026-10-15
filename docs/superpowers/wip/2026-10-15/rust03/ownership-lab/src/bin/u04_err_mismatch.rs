fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(s);
    println!("{word}");
}
