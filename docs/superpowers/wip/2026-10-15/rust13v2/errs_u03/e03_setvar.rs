//! 第13巻 UNIT 03 エラー3: edition 2024 で、環境変数を unsafe なしで書き換えようとした。
//! 実行: cargo run -q --example e03_setvar

use rustnote::config::Config;

fn main() {
    std::env::set_var("DATABASE_URL", "data/rustnote.db");
    println!("{:?}", Config::from_env());
}
