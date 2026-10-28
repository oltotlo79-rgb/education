//! DB の今の状態（適用済みの版・表・索引）を、migrate をかけずに表示する。
//! 実行: cargo run -q --example u02_state -- try.db

use libsql::Connection;
use rustnote::db::Db;

/// 1列目の文字列を、行の数だけ集める（表が無ければ空にする）
async fn strings(conn: &Connection, sql: &str) -> Vec<String> {
    let Ok(mut rows) = conn.query(sql, ()).await else {
        return Vec::new();
    };
    let mut values = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        if let Ok(value) = row.get::<String>(0) {
            values.push(value);
        }
    }
    values
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "try.db".to_string());
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    println!(
        "適用済みの版: {:?}",
        strings(
            &conn,
            "SELECT version FROM schema_migrations ORDER BY version"
        )
        .await
    );
    println!(
        "表: {:?}",
        strings(
            &conn,
            "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name"
        )
        .await
    );
    println!(
        "索引: {:?}",
        strings(
            &conn,
            "SELECT name FROM sqlite_master WHERE type = 'index' AND name LIKE 'idx_%' ORDER BY name"
        )
        .await
    );
    Ok(())
}
