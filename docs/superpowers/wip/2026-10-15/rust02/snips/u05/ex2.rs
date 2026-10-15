// text の中に target が何回出てくるかを数える
fn count_char(text: &str, target: char) -> usize {
    let mut count = 0;
    for c in text.chars() {
        if c == target {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_char("あいあいあ", 'あ'));
    println!("{}", count_char("Rust", 'x'));
}
