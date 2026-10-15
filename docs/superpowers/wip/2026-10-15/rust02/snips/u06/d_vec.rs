fn main() {
    // Vec::new() で空の Vec を作る。後から要素を足すので mut を付ける
    let mut titles: Vec<String> = Vec::new();
    titles.push(String::from("牛乳を買う"));
    titles.push(String::from("Rust の勉強"));
    titles.push(String::from("部屋の掃除"));
    println!("{titles:?}");
    println!("件数: {}", titles.len());

    // pop は最後の要素を取り出す（空なら None）
    let last = titles.pop();
    println!("取り出した: {last:?}");
    println!("残り: {titles:?}");

    // get は安全な取り出し。範囲外なら None
    println!("1番目: {:?}", titles.get(1));
    println!("9番目: {:?}", titles.get(9));

    // vec! マクロで、最初から中身のある Vec を作る
    let mut numbers = vec![5, 3, 8, 1];
    numbers.sort();
    println!("並べ替え: {numbers:?}");
    println!("8 を含む? {}", numbers.contains(&8));
    // remove(i) は i 番目を取り除いて、後ろを前に詰める
    numbers.remove(0);
    println!("先頭を削除: {numbers:?}");
}
