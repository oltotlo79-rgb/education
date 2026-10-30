//! 第9巻 UNIT 06: 環境変数から Config を作り、{:?} で表示する（トークンは表示されない）。
//! 実行: cargo run --example u06_config

use rustnote::config::{Config, listen_address};

fn main() {
    match Config::from_env() {
        Ok(config) => {
            println!("設定: {config:?}");
            println!("リモートの DB か: {}", config.is_remote());
        }
        Err(error) => println!("設定を作れません: {error}"),
    }
    println!("待ち受け: {}", listen_address());
}
