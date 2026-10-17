fn main() {
    for text in ["こんにちは", "Hello"] {
        println!(
            "{text}: {} バイト, {} 文字",
            text.len(),
            text.chars().count()
        );
    }
}
