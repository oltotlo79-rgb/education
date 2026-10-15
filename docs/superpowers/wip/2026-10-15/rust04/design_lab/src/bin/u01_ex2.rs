fn main() {
    let title = "日本語";
    // len() は UTF-8 のバイト数
    println!("len() = {}", title.len());
    // chars().count() は文字の数
    println!("chars().count() = {}", title.chars().count());
}
