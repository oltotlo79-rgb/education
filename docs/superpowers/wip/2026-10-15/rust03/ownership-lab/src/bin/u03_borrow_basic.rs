// &String は「String を借りて読むだけ」という意味
fn count_chars(text: &String) -> usize {
    text.chars().count()
}

// &mut String は「String を借りて書き換える」という意味
fn add_suffix(text: &mut String) {
    text.push_str("（編集済み）");
}

fn main() {
    let mut memo = String::from("牛乳を買う");

    // 共有参照（読むための借用）は、同時にいくつ作ってもよい
    let r1 = &memo;
    let r2 = &memo;
    println!("r1 = {r1}, r2 = {r2}");
    println!("文字数 = {}", count_chars(&memo));

    // 可変参照（書くための借用）で、中身を書き換える
    add_suffix(&mut memo);
    println!("memo = {memo}");

    let mut count = 10;
    let c = &mut count;
    // 参照の「先にある値」を変えるときは * を付ける
    *c += 1;
    println!("count = {count}");
}
