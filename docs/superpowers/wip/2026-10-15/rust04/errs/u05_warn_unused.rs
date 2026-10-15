fn save_note(title: &str) -> Result<(), String> {
    if title.is_empty() {
        return Err("タイトルが空です".to_string());
    }
    Ok(())
}

fn main() {
    // 結果を見ずに捨てている
    save_note("");
    println!("保存しました！");
}
