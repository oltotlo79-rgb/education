use std::mem::size_of;

// &str を受け取れば、String からも文字列リテラルからも呼べる
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}

// &[i32] を受け取れば、配列からも Vec からも、その一部分からも呼べる
fn total(scores: &[i32]) -> i32 {
    let mut sum = 0;
    for s in scores {
        sum += s;
    }
    sum
}

fn main() {
    let owned = String::from("hello rust world");
    let literal = "good morning";
    println!("{}", first_word(&owned));
    println!("{}", first_word(literal));

    // String の一部分（6バイト目から10バイト目の手前まで）を借りる
    let part = &owned[6..10];
    println!("part = {part} / len = {}", part.len());

    let array = [90, 75, 60];
    let vec = vec![10, 20, 30, 40];
    println!("配列の合計 = {}", total(&array));
    println!("Vec の合計 = {}", total(&vec));
    println!("Vec の一部の合計 = {}", total(&vec[1..3]));

    // スライス参照は「場所 + 長さ」の2つ組なので、ふつうの参照の2倍の大きさ
    println!("&String の大きさ: {} バイト", size_of::<&String>());
    println!("&str の大きさ: {} バイト", size_of::<&str>());
    println!("&[i32] の大きさ: {} バイト", size_of::<&[i32]>());
}
