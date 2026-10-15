fn main() {
    let text = "aあ🦀";
    // len() は「バイト数」を返す
    println!("len(): {}", text.len());
    // chars().count() は「文字数」を返す
    println!("chars().count(): {}", text.chars().count());

    // 1文字ずつ取り出して、それぞれが何バイトかを調べる
    for c in text.chars() {
        println!("'{c}' は {} バイト", c.len_utf8());
    }
    // 中身のバイトを16進数で並べて表示する
    println!("bytes: {:02X?}", text.as_bytes());
}
