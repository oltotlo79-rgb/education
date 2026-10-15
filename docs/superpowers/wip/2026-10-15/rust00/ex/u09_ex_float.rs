fn main() {
    let price: u32 = 120;
    // 税込み価格（1.08 倍）を計算したい
    let with_tax = price * 1.08;
    println!("税込み: {with_tax}");
}
