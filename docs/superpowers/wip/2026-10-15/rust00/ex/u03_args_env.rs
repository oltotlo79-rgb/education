use std::env;

fn main() {
    // コマンドラインから渡された値を順番に表示する（0 番目はプログラム自身）
    for (i, arg) in env::args().enumerate() {
        println!("引数 {i}: {arg}");
    }

    // 環境変数 GREETING を読む（設定されていなければ「こんにちは」を使う）
    let greeting = env::var("GREETING").unwrap_or(String::from("こんにちは"));
    println!("あいさつ: {greeting}");
}
