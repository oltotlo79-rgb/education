// 年齢の文字列を検証して u8 にする
fn parse_age(raw: &str) -> Result<u8, String> {
    let age: u8 = raw
        .trim()
        .parse()
        .map_err(|_| format!("「{raw}」は 0〜255 の数ではありません"))?;
    if age > 150 {
        return Err(format!("{age} 歳は大きすぎます"));
    }
    Ok(age)
}

fn main() {
    for raw in ["20", "abc", "200", "300"] {
        match parse_age(raw) {
            Ok(age) => println!("OK: {age} 歳"),
            Err(message) => println!("NG: {message}"),
        }
    }
}
