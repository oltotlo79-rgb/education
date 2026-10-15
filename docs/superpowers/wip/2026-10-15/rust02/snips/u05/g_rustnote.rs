const MAX_TITLE_CHARS: usize = 80;

// 文字数が 1〜80 の範囲なら true
fn is_valid_title(title: &str) -> bool {
    let count = title.chars().count();
    (1..=MAX_TITLE_CHARS).contains(&count)
}

fn main() {
    // 「あ」を 80 文字並べたタイトル（ちょうど上限）
    let title = "あ".repeat(80);
    println!("len()           = {} バイト", title.len());
    println!("chars().count() = {} 文字", title.chars().count());

    // もし len() で判定していたら、上限ちょうどの日本語タイトルを拒否してしまう
    let by_len = title.len() <= MAX_TITLE_CHARS;
    println!("len() で判定:           {by_len}");
    println!("chars().count() で判定: {}", is_valid_title(&title));

    // 前後の空白を取り除いてから判定する
    println!("空白だけ: {}", is_valid_title("   ".trim()));
}
