// 長いタイトルを max 文字に切り詰め、切ったときは「…」を付ける
fn preview(title: &str, max: usize) -> String {
    if title.chars().count() <= max {
        return title.to_string();
    }
    let head: String = title.chars().take(max).collect();
    format!("{head}…")
}

// 連続した空白（全角スペースやタブも含む）を、半角スペース1つにまとめる
fn squash_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn main() {
    println!("{}", preview("短いタイトル", 10));
    println!("{}", preview("とても長いタイトルなので一覧では省略したい", 10));
    println!("[{}]", squash_spaces("  牛乳 と　　卵を\t買う  "));
}
