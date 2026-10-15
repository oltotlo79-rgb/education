// 小さい方と大きい方を、タプルにまとめて2つ同時に返す
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }
}

fn main() {
    // 型の違う値を、決まった個数だけひとまとめにできる
    let note: (u64, &str, bool) = (1, "牛乳を買う", false);
    // .0 .1 .2 で、何番目かを指定して取り出す
    println!("ID: {}, タイトル: {}, 完了: {}", note.0, note.1, note.2);

    // 分解して、それぞれに名前を付ける
    let (id, title, done) = note;
    println!("{id} / {title} / {done}");

    let (low, high) = min_max(9, 3);
    println!("小さい方: {low}, 大きい方: {high}");
}
