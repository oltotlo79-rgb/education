fn main() {
    let title = String::from("牛乳を買う");
    // clone() で「複製」を作って saved に渡す。title は手元に残る
    let saved = title.clone();
    println!("保存したタイトル: {saved}");
    println!("元のタイトル: {title}");
}
