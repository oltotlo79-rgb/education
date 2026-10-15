fn main() {
    let s = "あいう";
    // len() は「バイト数」。日本語のひらがなは1文字3バイト
    println!("len = {}", s.len());
    println!("文字数 = {}", s.chars().count());
    // 0〜3バイト目は「あ」の1文字ぶん
    println!("&s[0..3] = {}", &s[0..3]);
    // 文字の区切りと、その位置（バイト）を順に取り出す
    for (byte_pos, ch) in s.char_indices() {
        println!("{byte_pos} バイト目から: {ch}");
    }
}
