//! 第13巻 UNIT 05 エラー2: 接続を作る connect() に、.await を付け忘れた。
//! 実行: cargo run -q --example e05_await

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("e05.db")).await?;
    let conn: libsql::Connection = db.connect()?;
    conn.execute("SELECT 1", ()).await?;
    Ok(())
}
