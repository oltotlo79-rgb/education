//! RustNote の db::migrate を同じ DB に2回かけて、何が作られ、2回目に何が起きるかを確かめる。
//! DB ファイルの場所を引数で渡す（無ければ作られる）。
//! 実行: cargo run -q --example u02_migrate -- try.db

use libsql::Connection;
use rustnote::db::{self, Db};

/// 1列目の文字列を、行の数だけ集める
async fn strings(conn: &Connection, sql: &str) -> anyhow::Result<Vec<String>> {
    let mut rows = conn.query(sql, ()).await?;
    let mut values = Vec::new();
    while let Some(row) = rows.next().await? {
        values.push(row.get::<String>(0)?);
    }
    Ok(values)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "try.db".to_string());
    let db = Db::open_local(&path).await?;

    println!("1回目の migrate: {:?}", db::migrate(&db).await?);
    println!("2回目の migrate: {:?}", db::migrate(&db).await?);

    let conn = db.connect().await?;
    println!(
        "適用済みの版: {:?}",
        strings(
            &conn,
            "SELECT version FROM schema_migrations ORDER BY version"
        )
        .await?
    );
    println!(
        "表: {:?}",
        strings(
            &conn,
            "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name"
        )
        .await?
    );
    println!(
        "索引: {:?}",
        strings(
            &conn,
            "SELECT name FROM sqlite_master WHERE type = 'index' AND name LIKE 'idx_%' ORDER BY name"
        )
        .await?
    );
    println!(
        "notes の列: {:?}",
        strings(&conn, "SELECT name FROM pragma_table_info('notes')").await?
    );
    Ok(())
}
