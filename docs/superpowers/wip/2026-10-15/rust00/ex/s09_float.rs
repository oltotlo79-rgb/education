fn main() {
    let price: u32 = 120;

    // 方法 1: 整数のまま「108 を掛けて 100 で割る」（小数点以下は切り捨て）
    let with_tax = price * 108 / 100;
    println!("税込み（整数で計算）: {with_tax}");

    // 方法 2: f64（小数の型）に変換してから 1.08 倍する
    let with_tax_f64 = f64::from(price) * 1.08;
    println!("税込み（小数で計算）: {with_tax_f64}");
}
