fn main() {
    let raw = "  Rust, Web,  DB  ";
    // trim: 前後の空白を取り除く
    println!("[{}]", raw.trim());
    // to_lowercase: 小文字にする（to_uppercase なら大文字）
    println!("{}", raw.trim().to_lowercase());
    // contains / starts_with: 含むか・その文字で始まるか
    println!("Web を含む? {}", raw.contains("Web"));
    println!("Rust で始まる? {}", raw.trim().starts_with("Rust"));
    // replace: 置き換えた新しい String を作る
    println!("{}", raw.trim().replace(", ", " / "));
    // split: 区切り文字で分ける。各部分の空白も trim で取り除く
    for part in raw.split(',') {
        println!("- [{}]", part.trim());
    }
    // lines: 改行で分ける（\n は改行を表す）
    let memo = "1行目\n2行目\n3行目";
    println!("行数: {}", memo.lines().count());
}
