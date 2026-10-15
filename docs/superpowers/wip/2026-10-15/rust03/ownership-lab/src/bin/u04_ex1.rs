fn last_word(text: &str) -> &str {
    match text.rfind(' ') {
        Some(pos) => &text[pos + 1..],
        None => text,
    }
}

fn main() {
    let owned = String::from("hello rust world");
    println!("{}", last_word(&owned));
    println!("{}", last_word("single"));
}
