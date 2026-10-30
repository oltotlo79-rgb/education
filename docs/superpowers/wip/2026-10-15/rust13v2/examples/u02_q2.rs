//! 問2の解答: 一覧の SQL が、索引 idx_notes_user_updated を使っているかを EXPLAIN QUERY PLAN で確かめる。
//! 実行: cargo run -q --example u02_q2

use rustnote::db::{self, Db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u02.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;

    // src/repository/notes.rs の一覧の SQL と同じ形（列は短くしている）
    let list_sql = r"SELECT id, title FROM notes
        WHERE user_id = ?1 AND (title LIKE ?2 ESCAPE '\' OR body LIKE ?2 ESCAPE '\')
        ORDER BY updated_at DESC, id DESC
        LIMIT ?3 OFFSET ?4";
    for (label, sql) in [
        ("RustNote の一覧", list_sql.to_string()),
        (
            "持ち主の条件が無い一覧",
            "SELECT id, title FROM notes ORDER BY updated_at DESC, id DESC LIMIT 20".to_string(),
        ),
    ] {
        println!("[{label}]");
        let mut rows = conn.query(&format!("EXPLAIN QUERY PLAN {sql}"), ()).await?;
        while let Some(row) = rows.next().await? {
            println!("    {}", row.get::<String>(3)?);
        }
    }
    Ok(())
}
