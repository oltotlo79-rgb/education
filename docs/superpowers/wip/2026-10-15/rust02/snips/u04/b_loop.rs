fn main() {
    let mut n = 1;
    // loop は break するまで、いつまでもくり返す
    let result = loop {
        n *= 2;
        // 100 を超えたら、その値を持って loop を抜ける
        if n > 100 {
            break n;
        }
    };
    println!("100 を超えた最初の 2 の累乗: {result}");
}
