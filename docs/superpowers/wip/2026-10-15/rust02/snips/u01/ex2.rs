fn main() {
    let price = "1200";
    // 文字列を数に変換し、同じ名前で作り直す
    let price: u32 = price.parse().expect("数字ではありません");
    // 10% の税を足す（整数のまま計算するため、先に 110 を掛けてから 100 で割る）
    let price = price * 110 / 100;
    println!("税込み: {price} 円");
}
