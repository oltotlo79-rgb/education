//! Db::connect が接続ごとに入れている設定（PRAGMA）を、libsql の素の接続と比べる。
//! 実行: cargo run -q --example u02_pragmas

use libsql::{Builder, Connection, Value};
use rustnote::db::Db;

/// PRAGMA の値を1つ読む（数のときも文字のときもあるので、両方を文字にする）
async fn pragma(conn: &Connection, name: &str) -> anyhow::Result<String> {
    let mut rows = conn.query(&format!("PRAGMA {name}"), ()).await?;
    let row = rows.next().await?.expect("PRAGMA は1行を返すはずです");
    Ok(match row.get_value(0)? {
        Value::Integer(number) => number.to_string(),
        Value::Text(text) => text,
        other => format!("{other:?}"),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("u02.db");

    // RustNote の Db を通した接続
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    println!("[Db::connect の接続]");
    for name in ["journal_mode", "foreign_keys", "busy_timeout"] {
        println!("    {name} = {}", pragma(&conn, name).await?);
    }

    // libsql の素の接続（RustNote の設定を通さない）
    let raw = Builder::new_local(&path).build().await?.connect()?;
    println!("[libsql の素の接続]");
    for name in ["journal_mode", "foreign_keys", "busy_timeout"] {
        println!("    {name} = {}", pragma(&raw, name).await?);
    }

    // WAL モードにすると、DB の横に作業用のファイルができる
    let mut files: Vec<String> = std::fs::read_dir(dir.path())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    files.sort();
    println!("フォルダの中のファイル: {files:?}");
    Ok(())
}
