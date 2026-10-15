fn main() {
    let mut squares = Vec::new();
    for n in 1..=5 {
        squares.push(n * n);
    }
    println!("{squares:?}");
    let last = squares.pop();
    println!("取り出した: {last:?}, 残り: {squares:?}");
}
