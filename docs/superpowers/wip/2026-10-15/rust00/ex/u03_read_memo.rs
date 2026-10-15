use std::fs;

fn main() -> std::io::Result<()> {
    // memo.txt を UTF-8 の文字列として読み込む
    let text = fs::read_to_string("memo.txt")?;
    println!("memo.txt の中身: {}", text.trim_end());
    Ok(())
}
