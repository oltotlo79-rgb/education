use std::num::ParseIntError;

// main も Result を返せる。Err で終わると、エラーを表示して「失敗」として終了する
fn main() -> Result<(), ParseIntError> {
    let count: u32 = "12".parse()?;
    println!("count = {count}");
    let broken: u32 = "12a".parse()?;
    println!("ここには来ない: {broken}");
    Ok(())
}
