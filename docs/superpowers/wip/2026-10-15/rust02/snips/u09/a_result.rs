fn main() {
    for raw in ["42", "-7", "abc", ""] {
        // parse は Result を返す。成功なら Ok(値)、失敗なら Err(理由)
        let result: Result<i32, std::num::ParseIntError> = raw.parse::<i32>();
        match result {
            Ok(n) => println!("{raw:?} → 数 {n}"),
            // {e} で、失敗の理由を人向けの文で表示する
            Err(e) => println!("{raw:?} → 失敗: {e}"),
        }
    }
    // {:?} だと、プログラマー向けの詳しい形で表示される
    println!("{:?}", "abc".parse::<i32>());
}
