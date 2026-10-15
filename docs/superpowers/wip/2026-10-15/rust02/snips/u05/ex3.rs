fn main() {
    let text = "Rust入門";
    // 1文字ずつ取り出し、逆順にして、String に集める
    let reversed: String = text.chars().rev().collect();
    println!("{reversed}");
}
