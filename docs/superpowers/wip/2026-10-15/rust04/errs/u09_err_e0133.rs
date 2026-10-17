use std::env;

fn main() {
    // テストの準備のつもりで、プログラムの中から環境変数を書き換える
    env::set_var("PORT", "8080");
    println!("{:?}", env::var("PORT"));
}
