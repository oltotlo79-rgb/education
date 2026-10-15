fn main() {
    for text in ["42", "4x2"] {
        // parse の結果は「成功(Ok)」か「失敗(Err)」のどちらか
        match text.parse::<i32>() {
            Ok(n) => println!("{text} -> 数値 {n}"),
            Err(e) => println!("{text} -> 変換できません（{e}）"),
        }
    }
}
