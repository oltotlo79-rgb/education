//! 問3の解答: まだ無いフォルダの中の DB ファイルを開こうとすると、どうなるか。
//! 実行: cargo run -q --example u02_q3

use rustnote::db::Db;

#[tokio::main]
async fn main() {
    let dir = tempfile::tempdir().expect("一時フォルダを作れません");
    // ファイルは無くてもよいが、フォルダ（data）は作っていない
    let path = dir.path().join("data").join("rustnote.db");
    match Db::open_local(&path).await {
        Ok(_) => println!("開けました"),
        Err(error) => println!("開けません: {error}"),
    }

    // フォルダを作ってから開くと、ファイルは自動で作られる
    std::fs::create_dir(dir.path().join("data")).expect("フォルダを作れません");
    match Db::open_local(&path).await {
        Ok(_) => println!(
            "フォルダを作ってから: 開けました（ファイルがある: {}）",
            path.exists()
        ),
        Err(error) => println!("フォルダを作ってから: 開けません: {error}"),
    }
}
