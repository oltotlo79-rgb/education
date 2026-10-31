//! 第12巻 UNIT 10: DB のファイルに、接続を1つ開いたままにする（ほかの道具が DB を開いている状態を作る）。
//! 実行: cargo run -q --example u10_hold -- 調べる.db 秒数

use std::time::Duration;

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "rustnote.db".to_string());
    let seconds: u64 = args.next().unwrap_or_else(|| "20".to_string()).parse()?;
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let count = match rows.next().await? {
        Some(row) => row.get::<i64>(0)?,
        None => 0,
    };
    drop(rows);
    println!("notes: {count} 行。接続を開いたまま {seconds} 秒待ちます");
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    drop(conn);
    Ok(())
}
