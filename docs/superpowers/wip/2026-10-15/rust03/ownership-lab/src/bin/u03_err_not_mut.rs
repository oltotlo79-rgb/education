fn add_suffix(text: &mut String) {
    text.push_str("!");
}

fn main() {
    let memo = String::from("メモ");
    add_suffix(&mut memo);
    println!("{memo}");
}
