//! 秘密の値（Secret）を、{} でそのまま表示しようとした
use rustnote::config::Config;

fn main() {
    let config = Config::from_env().expect("設定を作れません");
    if let Some(token) = &config.database_auth_token {
        println!("トークン: {token}");
    }
}
