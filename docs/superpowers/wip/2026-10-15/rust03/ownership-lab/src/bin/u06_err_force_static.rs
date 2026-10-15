fn remember(cache: &mut Vec<&'static str>, text: &str) {
    cache.push(text);
}

fn main() {
    let mut cache = Vec::new();
    let input = String::from("一時的な文字列");
    remember(&mut cache, &input);
    println!("{cache:?}");
}
