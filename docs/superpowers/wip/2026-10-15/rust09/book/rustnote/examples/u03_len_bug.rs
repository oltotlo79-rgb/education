//! 文字数を len()（バイト数）で数えてしまう間違いを、chars().count() と比べる。
//! 実行: cargo run -q --example u03_len_bug

// 間違い: len() はバイト数を返す
fn is_valid_by_bytes(title: &str) -> bool {
    let len = title.trim().len();
    (1..=80).contains(&len)
}

// 正しい: chars().count() は文字数を返す（SQLite の length() と同じ数え方）
fn is_valid_by_chars(title: &str) -> bool {
    let len = title.trim().chars().count();
    (1..=80).contains(&len)
}

fn main() {
    let title = "あ".repeat(27);
    println!("「あ」27 個のタイトル");
    println!(
        "    len()           = {} → {}",
        title.len(),
        is_valid_by_bytes(&title)
    );
    println!(
        "    chars().count() = {} → {}",
        title.chars().count(),
        is_valid_by_chars(&title)
    );
}
