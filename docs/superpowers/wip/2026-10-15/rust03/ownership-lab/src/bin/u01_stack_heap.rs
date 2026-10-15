use std::mem::size_of;

fn main() {
    // 整数は大きさが決まっているので、値そのものがスタックに置かれる
    let n: i32 = 5;
    // String は「持ち手」(ptr・len・cap) がスタックに、文字の本体がヒープに置かれる
    let mut s = String::from("hello");

    println!("n = {n}");
    println!("s = {s} / len = {} / cap = {}", s.len(), s.capacity());

    // 文字を足すと、ヒープの本体が大きくなる
    s.push_str(", world");
    println!("s = {s} / len = {} / cap = {}", s.len(), s.capacity());

    // 持ち手の大きさは、中身の文字数に関係なくいつも同じ
    println!("i32 の大きさ: {} バイト", size_of::<i32>());
    println!("String の持ち手の大きさ: {} バイト", size_of::<String>());
}
