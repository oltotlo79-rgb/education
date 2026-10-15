fn main() {
    // 在庫の数
    let mut stock = HashMap::new();
    stock.insert("りんご", 3);

    // 値段
    let mut prices = HashMap::new();
    prices.insert("りんご", 120);

    println!("{stock:?} {prices:?}");
}
