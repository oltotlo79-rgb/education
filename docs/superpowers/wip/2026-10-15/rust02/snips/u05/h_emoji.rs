fn main() {
    // 家族の絵文字は、3人の絵文字を「つなぎ文字」でつないだもの
    let family = "👨‍👩‍👧";
    println!("見た目は1文字: {family}");
    println!("chars().count() = {}", family.chars().count());
    println!("len() = {}", family.len());
}
