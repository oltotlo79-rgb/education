use std::fmt;

// Vec<String> を包んだ、自分の crate の型を作る
struct TitleList(Vec<String>);

// 自分の型なら、標準ライブラリの trait を実装できる
impl fmt::Display for TitleList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    let titles = TitleList(vec!["牛乳を買う".to_string(), "本を返す".to_string()]);
    println!("{titles}");
}
