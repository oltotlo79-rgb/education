//! 同じタイトルを「型の検査（NoteTitle::parse）」と「DB の制約（CHECK）」の両方に渡し、
//! 2つの守りの答えが食い違う入力を探す。
//! 実行: cargo run -q --example u03_layers

use rustnote::db::{self, Db};
use rustnote::domain::NoteTitle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
        (),
    )
    .await?;

    let eighty_one = "あ".repeat(81);
    let inputs = [
        ("牛乳を買う", "牛乳を買う"),
        ("空", ""),
        ("空白3つ", "   "),
        ("全角の空白1つ", "\u{3000}"),
        ("ゼロ幅スペース1つ", "\u{200B}"),
        ("「あ」81個", eighty_one.as_str()),
    ];
    for (label, input) in inputs {
        // 型の検査
        let by_type = match NoteTitle::parse(input) {
            Ok(_) => "OK".to_string(),
            Err(error) => format!("NG: {error}"),
        };
        // DB の制約（検査を通さず、入力をそのまま INSERT する）
        let by_db = match conn
            .execute("INSERT INTO notes (user_id, title) VALUES (1, ?1)", [input])
            .await
        {
            Ok(_) => "OK".to_string(),
            Err(error) => format!("NG: {error}"),
        };
        let same = by_type.starts_with("OK") == by_db.starts_with("OK");
        println!("[{label}]");
        println!("    型: {by_type}");
        println!("    DB: {by_db}");
        if !same {
            println!("    → 答えが食い違う");
        }
    }
    Ok(())
}
