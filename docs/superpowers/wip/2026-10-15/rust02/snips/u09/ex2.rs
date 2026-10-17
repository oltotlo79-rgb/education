use std::num::ParseIntError;

// 「時間」と「分」の文字列から、合計の分数を求める
fn total_minutes(hours: &str, minutes: &str) -> Result<u32, ParseIntError> {
    let h: u32 = hours.parse()?;
    let m: u32 = minutes.parse()?;
    Ok(h * 60 + m)
}

fn main() {
    println!("{:?}", total_minutes("2", "30"));
    println!("{:?}", total_minutes("2", "半"));
}
