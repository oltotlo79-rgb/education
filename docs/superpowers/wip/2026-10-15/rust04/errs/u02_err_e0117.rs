use std::fmt;

// 標準ライブラリの trait（Display）を、標準ライブラリの型（Vec<String>）に実装しようとする
impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.join(", "))
    }
}

fn main() {
    let titles = vec!["牛乳を買う".to_string(), "本を返す".to_string()];
    println!("{titles}");
}
