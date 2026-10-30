//! 第13巻 UNIT 05 問1: LIKE の % と _ を、escape せずに検索したら、どうなるか。
//! 実行: cargo run -q --example u05_q1

use libsql::Connection;
use rustnote::db::{self, Db};

/// LIKE で title を探し、当たったタイトルを集める
async fn titles(conn: &Connection, sql: &str, pattern: &str) -> anyhow::Result<Vec<String>> {
    let mut rows = conn.query(sql, [pattern]).await?;
    let mut found = Vec::new();
    while let Some(row) = rows.next().await? {
        found.push(row.get::<String>(0)?);
    }
    Ok(found)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u05_q1.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
        (),
    )
    .await?;
    for title in ["100%オフ", "1000円", "a_b の計算", "acb の計算"] {
        conn.execute(
            "INSERT INTO notes (user_id, title, body) VALUES (1, ?1, '')",
            [title],
        )
        .await?;
    }

    let plain = "SELECT title FROM notes WHERE title LIKE ?1 ORDER BY id";
    let escaped = "SELECT title FROM notes WHERE title LIKE ?1 ESCAPE '\\' ORDER BY id";
    for (keyword, raw, safe) in [("100%", "%100%%", "%100\\%%"), ("a_b", "%a_b%", "%a\\_b%")] {
        println!("キーワード 「{keyword}」");
        println!(
            "  escape なし 「{raw}」 → {:?}",
            titles(&conn, plain, raw).await?
        );
        println!(
            "  escape あり 「{safe}」 → {:?}",
            titles(&conn, escaped, safe).await?
        );
    }
    Ok(())
}
