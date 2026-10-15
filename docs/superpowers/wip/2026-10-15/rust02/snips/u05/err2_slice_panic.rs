fn main() {
    let title = String::from("日本語");
    // 先頭の2バイトを取り出そうとする
    let part = &title[0..2];
    println!("{part}");
}
