fn main() {
    let names = vec![String::from("佐藤"), String::from("鈴木")];
    // 1. 借りる（いちばん安い）
    let first: &String = &names[0];
    println!("借りた: {first}");
    // 2. 複製する（names は残る）
    let copied: String = names[0].clone();
    println!("複製した: {copied}");
    // 3. Vec ごと手放して、要素を取り出す
    let mut names = names;
    let taken: String = names.remove(0);
    println!("取り出した: {taken} / 残り: {names:?}");
}
