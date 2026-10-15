fn main() {
    let a: u32 = 80;
    let b: u32 = 95;
    let c: u32 = 72;
    // 合計は整数のまま計算する
    let sum = a + b + c;
    // 割る直前に f64 へ変換する
    let average = sum as f64 / 3.0;
    println!("平均: {average:.1} 点");
}
