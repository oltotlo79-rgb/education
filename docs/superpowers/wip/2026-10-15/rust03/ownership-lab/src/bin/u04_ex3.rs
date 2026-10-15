fn initials(name: &str) -> String {
    let mut result = String::new();
    for word in name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            result.push(first);
        }
    }
    result
}

fn main() {
    println!("{}", initials("Grace Brewster Hopper"));
    println!("{}", initials("山田 太郎"));
}
