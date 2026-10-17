fn main() {
    // 1〜10 のうち奇数だけを2乗して合計する
    let total: u32 = (1..=10).filter(|n| n % 2 == 1).map(|n| n * n).sum();
    println!("{total}");
}
