fn main() {
    let mut s = String::from("メモ");
    let w1 = &mut s;
    let w2 = &mut s;
    w1.push_str("A");
    w2.push_str("B");
    println!("{s}");
}
