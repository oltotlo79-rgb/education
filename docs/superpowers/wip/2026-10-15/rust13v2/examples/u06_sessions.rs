//! 第13巻 UNIT 06: sessions の表の中身を表示する（サーバーが動いている間に、別に開いて読む）。
//! 実行: cargo run -q --example u06_sessions -- u06.db

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "u06.db".to_string());
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    let mut rows = conn
        .query(
            "SELECT u.email, length(s.token_hash), typeof(s.token_hash), s.expires_at - unixepoch()
               FROM sessions s JOIN users u ON u.id = s.user_id
              ORDER BY s.rowid",
            (),
        )
        .await?;
    let mut count = 0;
    while let Some(row) = rows.next().await? {
        count += 1;
        println!(
            "  {}・トークンのハッシュ {} バイト（{}）・期限まで {} 秒",
            row.get::<String>(0)?,
            row.get::<i64>(1)?,
            row.get::<String>(2)?,
            row.get::<i64>(3)?
        );
    }
    println!("sessions の行: {count} 件");
    Ok(())
}
