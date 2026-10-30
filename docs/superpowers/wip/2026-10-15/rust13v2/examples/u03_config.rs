//! 第13巻 UNIT 03: 環境変数の組み合わせごとに、Config::from_env がどう読むかを確かめる。
//! 実行: cargo run -q --example u03_config

use std::env;

use rustnote::config::Config;

/// 環境変数をそろえてから、Config::from_env を呼んで結果を表示する
fn try_with(label: &str, url: Option<&str>, token: Option<&str>) {
    // SAFETY: このプログラムは1つのスレッドだけで動き、ほかに環境変数を読む人がいない
    unsafe {
        match url {
            Some(value) => env::set_var("DATABASE_URL", value),
            None => env::remove_var("DATABASE_URL"),
        }
        match token {
            Some(value) => env::set_var("DATABASE_AUTH_TOKEN", value),
            None => env::remove_var("DATABASE_AUTH_TOKEN"),
        }
    }
    match Config::from_env() {
        Ok(config) => println!(
            "{label}\n  → Ok: {config:?}（remote: {}）",
            config.is_remote()
        ),
        Err(error) => println!("{label}\n  → Err: {error}"),
    }
}

fn main() {
    try_with("1. 何も設定しない", None, None);
    try_with("2. ローカルのファイル", Some("data/rustnote.db"), None);
    try_with(
        "3. Turso の URL・トークン無し",
        Some("libsql://rustnote-demo.turso.io"),
        None,
    );
    try_with(
        "4. Turso の URL・トークンが空",
        Some("libsql://rustnote-demo.turso.io"),
        Some(""),
    );
    try_with(
        "5. Turso の URL・トークンあり",
        Some("libsql://rustnote-demo.turso.io"),
        Some("very-secret-token"),
    );
    try_with(
        "6. 手元の sqld（http）・トークン無し",
        Some("http://127.0.0.1:8080"),
        None,
    );
}
