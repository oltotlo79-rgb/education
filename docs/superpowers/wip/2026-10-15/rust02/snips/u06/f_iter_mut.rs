fn main() {
    let mut prices = vec![100, 250, 980];
    // &mut で借りると、要素を書き換えながら回せる
    for price in &mut prices {
        // *price は「借りている先の値そのもの」
        *price = *price * 110 / 100;
    }
    println!("{prices:?}");

    // 何番目かも一緒に欲しいときは enumerate を使う
    for (i, price) in prices.iter().enumerate() {
        println!("{i}: {price} 円");
    }
}
