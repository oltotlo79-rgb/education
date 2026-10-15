fn main() {
    // 配列: 同じ型の値を、決まった個数だけ並べる。型は [要素の型; 個数]
    let scores: [u32; 5] = [72, 88, 95, 60, 81];
    // [ ] の中の数（添え字）は 0 から数える
    println!("先頭: {}", scores[0]);
    println!("最後: {}", scores[scores.len() - 1]);
    println!("個数: {}", scores.len());

    // [初期値; 個数] で、同じ値を並べた配列を作れる
    let zeros = [0; 4];
    // {:?} で配列をまとめて表示する
    println!("{zeros:?}");

    // mut を付ければ、要素を書き換えられる（個数は変えられない）
    let mut week = ["月", "火", "水"];
    week[1] = "木";
    println!("{week:?}");
}
