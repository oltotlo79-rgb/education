fn main() {
    let ascii = "Rustacean";
    // &文字列[開始..終了] で一部分（スライス）を取り出す。数字は「バイトの位置」
    println!("{}", &ascii[0..4]);

    let japanese = "日本語のノート";
    // 日本語は1文字3バイトなので、0..6 は先頭の2文字
    println!("{}", &japanese[0..6]);

    // 「文字数」で切り出したいときは chars() を使う
    let first_three: String = japanese.chars().take(3).collect();
    println!("{first_three}");

    // その位置が文字の区切りかどうかを調べられる
    println!("2バイト目は文字の区切り? {}", japanese.is_char_boundary(2));
    // get は、区切りでない位置なら None を返す（panic しない）
    println!("{:?}", japanese.get(0..2));
    println!("{:?}", japanese.get(0..3));
}
