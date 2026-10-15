fn log(message: &str) {
    println!("[log] {message}");
}

fn main() {
    // 戻り値の型を書かない関数は、実は () という値を返している
    let result = log("起動しました");
    println!("log の戻り値: {result:?}");
}
