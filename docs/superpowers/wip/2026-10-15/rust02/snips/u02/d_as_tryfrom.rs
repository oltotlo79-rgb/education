fn main() {
    // as: 型を強制的に変換する。入りきらない分は黙って捨てられる
    let big: i32 = 300;
    let small = big as u8;
    println!("300 as u8   = {small}");
    let minus: i32 = -1;
    println!("-1 as u32   = {}", minus as u32);
    // 小数 → 整数は、小数点以下が切り捨てられる
    println!("3.99 as i32 = {}", 3.99_f64 as i32);

    // From: 必ず成功する変換（小さい型 → 大きい型）
    let level: u8 = 200;
    let wide = u32::from(level);
    println!("u32::from(200u8) = {wide}");

    // TryFrom: 失敗するかもしれない変換。結果は Ok(値) か Err(理由)
    println!("u8::try_from(300) = {:?}", u8::try_from(big));
    println!("u8::try_from(100) = {:?}", u8::try_from(100_i32));
}
