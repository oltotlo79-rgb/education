use std::num::ParseIntError;

// 2つの文字列を数にして足す。どちらかが数でなければ、その時点で Err を返す
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // ? は「Ok なら中身を取り出して続ける。Err ならその Err をすぐ返す」
    let x: i32 = a.trim().parse()?;
    let y: i32 = b.trim().parse()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", " 32 "));
    println!("{:?}", add_strings("10", "abc"));
}
