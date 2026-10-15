const MAX_TITLE_CHARS: usize = 80;

// 前後の空白を取り除いた版を返す
fn normalize(raw: &str) -> &str {
    raw.trim()
}

// 文字数を数える（日本語も 1 文字を 1 と数える）
fn char_count(text: &str) -> usize {
    text.chars().count()
}

// 1 以上 80 以下なら true を返す
fn is_valid_length(len: usize) -> bool {
    (1..=MAX_TITLE_CHARS).contains(&len)
}

// 判定結果を、人が読める文にして返す
fn describe(raw: &str) -> String {
    let title = normalize(raw);
    let len = char_count(title);
    if is_valid_length(len) {
        format!("OK: 「{title}」（{len}文字）")
    } else {
        format!("NG: {len}文字は 1〜{MAX_TITLE_CHARS} 文字の範囲外です")
    }
}

fn main() {
    println!("{}", describe("  買い物リスト  "));
    println!("{}", describe("    "));
    // "あ" を 81 回くり返した文字列で試す
    let long = "あ".repeat(81);
    println!("{}", describe(&long));
}
