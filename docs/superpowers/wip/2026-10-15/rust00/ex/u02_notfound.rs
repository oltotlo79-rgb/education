use std::fs;

fn main() -> std::io::Result<()> {
    // config.txt を読む（今いるフォルダに無ければ失敗する）
    let text = fs::read_to_string("config.txt")?;
    println!("設定: {text}");
    Ok(())
}
