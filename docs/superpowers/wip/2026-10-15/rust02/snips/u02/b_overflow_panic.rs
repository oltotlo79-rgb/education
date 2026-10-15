fn main() {
    // 文字列から作った値（コンパイルの時点では中身が分からない値）
    let stock: u8 = "255".parse().unwrap();
    println!("在庫: {stock}");
    // u8 の最大は 255。1 を足すと入りきらない
    let next = stock + 1;
    println!("次: {next}");
}
