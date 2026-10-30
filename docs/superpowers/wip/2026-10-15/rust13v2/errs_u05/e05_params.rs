//! 第13巻 UNIT 05 エラー3: SQL の値に、UserId をそのまま渡した。
//! 実行: cargo run -q --example e05_params

use rustnote::db::{self, Db};
use rustnote::domain::UserId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("e05.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    let owner = UserId::new(1)?;
    let mut rows: libsql::Rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = ?1",
            libsql::params![owner],
        )
        .await?;
    if let Some(row) = rows.next().await? {
        println!("{}", row.get::<i64>(0)?);
    }
    Ok(())
}
