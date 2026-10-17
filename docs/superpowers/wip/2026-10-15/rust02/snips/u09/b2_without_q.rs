use std::num::ParseIntError;

// ? を使わずに書いた add_strings（中身は同じ動き）
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = match a.trim().parse() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y: i32 = match b.trim().parse() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", " 32 "));
    println!("{:?}", add_strings("10", "abc"));
}
