// items のうち、page ページ目（1 始まり）の部分だけを貸して返す
fn page_of(items: &[String], page: usize, per_page: usize) -> &[String] {
    let start = (page - 1) * per_page;
    // 開始位置が件数以上なら、空のスライスを返す
    if start >= items.len() {
        return &[];
    }
    // 終わりの位置は、件数を超えないようにそろえる
    let end = (start + per_page).min(items.len());
    &items[start..end]
}

fn main() {
    let mut notes = Vec::new();
    for i in 1..=7 {
        notes.push(format!("ノート{i}"));
    }
    for page in 1..=4 {
        println!("{page}ページ目: {:?}", page_of(&notes, page, 3));
    }
}
