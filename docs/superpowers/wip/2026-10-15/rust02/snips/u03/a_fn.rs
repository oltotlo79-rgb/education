// 2つの整数を受け取り、その合計を返す関数
fn add(a: i32, b: i32) -> i32 {
    // 最後の行にセミコロンを付けないと、その値が戻り値になる
    a + b
}

// 何も返さない関数（-> 戻り値の型 を書かない）
fn greet(name: &str) {
    println!("こんにちは、{name}さん");
}

fn main() {
    let total = add(3, 4);
    println!("3 + 4 = {total}");
    greet("Rust");
    // 関数の呼び出し結果を、そのまま別の関数の引数にしてもよい
    println!("入れ子: {}", add(add(1, 2), add(3, 4)));
}
