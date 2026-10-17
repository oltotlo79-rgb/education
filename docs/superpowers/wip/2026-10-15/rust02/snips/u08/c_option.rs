// keyword を含む最初のタイトルの位置を返す。見つからなければ None
fn find_title(titles: &[&str], keyword: &str) -> Option<usize> {
    for (i, title) in titles.iter().enumerate() {
        if title.contains(keyword) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let titles = ["牛乳を買う", "Rust の勉強", "部屋の掃除"];

    // match で Some と None の両方の場合を書く
    match find_title(&titles, "Rust") {
        Some(i) => println!("{i} 番目に見つかりました: {}", titles[i]),
        None => println!("見つかりません"),
    }

    // if let: Some のときだけ処理したいとき
    if let Some(i) = find_title(&titles, "掃除") {
        println!("掃除は {i} 番目");
    }

    // unwrap_or: None のときに使う代わりの値を決める
    let pos = find_title(&titles, "旅行").unwrap_or(999);
    println!("旅行の位置（なければ 999）: {pos}");

    // map: Some の中身だけを変換する（None は None のまま）
    let shown = find_title(&titles, "牛乳").map(|i| i + 1);
    println!("1 から数えた位置: {shown:?}");

    // is_some: あるかないかだけを調べる
    println!("旅行はある? {}", find_title(&titles, "旅行").is_some());
}
