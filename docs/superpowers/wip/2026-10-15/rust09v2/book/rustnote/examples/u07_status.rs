//! 第9巻 UNIT 07: アプリが持つマイグレーションと、DB に記録された適用済みの版を並べて表示する。
//! 実行: $env:DATABASE_URL = "dev.db"; cargo run --example u07_status

use rustnote::{
    config::Config,
    db::{Db, MIGRATIONS},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let db = Db::open(&config).await?;
    let conn = db.connect().await?;

    // DB に記録された版（schema_migrations が無ければ、まだ1つも適用していない）
    let mut applied = Vec::new();
    if let Ok(mut rows) = conn
        .query(
            "SELECT version, applied_at FROM schema_migrations ORDER BY version",
            (),
        )
        .await
    {
        while let Some(row) = rows.next().await? {
            applied.push((row.get::<String>(0)?, row.get::<String>(1)?));
        }
    }

    println!("{}", config.database_url);
    for migration in MIGRATIONS {
        match applied
            .iter()
            .find(|(version, _)| version == migration.version)
        {
            Some((_, at)) => println!("  {}  適用済み（{at}）", migration.version),
            None => println!("  {}  まだ", migration.version),
        }
    }
    Ok(())
}
