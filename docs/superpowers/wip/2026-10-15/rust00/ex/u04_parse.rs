fn main() {
    // 文字列を数値に変換する。どちらもコンパイルは通る
    let ok: i32 = "42".parse().unwrap();
    println!("1 つ目: {ok}");

    // "4x2" は数値ではないので、実行してみて初めて失敗が分かる
    let ng: i32 = "4x2".parse().unwrap();
    println!("2 つ目: {ng}");
}
