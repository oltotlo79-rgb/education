//! 第12巻 UNIT 10: DB のファイルの中の、ユーザーとノートの数を数え、壊れていないかも確かめる（バックアップを確かめる）。
//! 実行: cargo run -q --example u10_count -- 調べる.db

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| "rustnote.db".to_string());
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    for table in ["users", "notes"] {
        let mut rows = conn
            .query(&format!("SELECT COUNT(*) FROM {table}"), ())
            .await?;
        let count = match rows.next().await? {
            Some(row) => row.get::<i64>(0)?,
            None => 0,
        };
        println!("{table}: {count} 行");
    }
    // 第7巻 UNIT 10: 壊れていなければ「ok」の1行。壊れていれば、見つかった問題が1行ずつ返る
    let mut rows = conn.query("PRAGMA integrity_check", ()).await?;
    let mut problems = Vec::new();
    while let Some(row) = rows.next().await? {
        problems.push(row.get::<String>(0)?);
    }
    match problems.as_slice() {
        [only] if only == "ok" => println!("integrity_check: ok"),
        _ => {
            println!("integrity_check: 問題が {} 件", problems.len());
            for problem in problems.iter().take(3) {
                println!("  {problem}");
            }
        }
    }
    Ok(())
}
