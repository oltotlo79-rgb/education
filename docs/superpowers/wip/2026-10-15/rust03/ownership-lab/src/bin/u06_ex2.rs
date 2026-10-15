fn remember(cache: &mut Vec<String>, text: &str) {
    cache.push(text.to_string());
}

fn main() {
    let mut cache = Vec::new();
    {
        let input = String::from("一時的な文字列");
        remember(&mut cache, &input);
    }
    println!("{cache:?}");
}
