//! 第13巻 UNIT 03 エラー2: Secret を {} で表示しようとした。
//! 実行: cargo run -q --example e03_display

use rustnote::config::Secret;

fn main() {
    let token = Secret::new("very-secret-token".to_string());
    println!("トークン: {}", token);
}
