use libsql::{Builder, Connection, Transaction};
use thiserror::Error;

// 1つのマイグレーション（版の名前と SQL）
struct Migration {
    version: &'static str,
    sql: &'static str,
}

// アプリが持つマイグレーションの一覧（古い順）。
// SQL ファイルは include_str! でコンパイル時に実行ファイルへ埋め込む
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: "0001_users",
        sql: include_str!("../../migrations/0001_users.sql"),
    },
    Migration {
        version: "0002_notes",
        sql: include_str!("../../migrations/0002_notes.sql"),
    },
    Migration {
        version: "0003_sessions",
        sql: include_str!("../../migrations/0003_sessions.sql"),
    },
];

#[derive(Debug, Error)]
enum MigrateError {
    #[error("schema_migrations を準備できません")]
    Prepare(#[source] libsql::Error),
    #[error("マイグレーション {version} に失敗しました")]
    Apply {
        version: &'static str,
        #[source]
        source: libsql::Error,
    },
}

// まだ適用していないマイグレーションだけを、1つずつトランザクションの中で適用する。
// 戻り値は、今回新しく適用した版の名前の一覧
async fn migrate(conn: &Connection) -> Result<Vec<&'static str>, MigrateError> {
    // 適用済みの版を記録する表（無ければ作る）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        (),
    )
    .await
    .map_err(MigrateError::Prepare)?;

    let applied = applied_versions(conn)
        .await
        .map_err(MigrateError::Prepare)?;

    let mut newly_applied = Vec::new();
    for migration in MIGRATIONS {
        if applied.iter().any(|v| v == migration.version) {
            continue;
        }
        let apply_error = |source| MigrateError::Apply {
            version: migration.version,
            source,
        };
        let tx = conn.transaction().await.map_err(apply_error)?;
        match apply(&tx, migration).await {
            // 全部成功したときだけ確定する
            Ok(()) => tx.commit().await.map_err(apply_error)?,
            // 途中で失敗したら、その版の変更をすべて取り消す
            Err(source) => {
                tx.rollback().await.map_err(apply_error)?;
                return Err(apply_error(source));
            }
        }
        newly_applied.push(migration.version);
    }
    Ok(newly_applied)
}

// schema_migrations に記録された版の名前を読む
async fn applied_versions(conn: &Connection) -> Result<Vec<String>, libsql::Error> {
    let mut rows = conn
        .query("SELECT version FROM schema_migrations ORDER BY version", ())
        .await?;
    let mut versions = Vec::new();
    while let Some(row) = rows.next().await? {
        versions.push(row.get::<String>(0)?);
    }
    Ok(versions)
}

// 1つのマイグレーションの SQL を実行し、適用済みとして記録する
async fn apply(tx: &Transaction, migration: &Migration) -> Result<(), libsql::Error> {
    tx.execute_batch(migration.sql).await?;
    tx.execute(
        "INSERT INTO schema_migrations (version) VALUES (?1)",
        [migration.version],
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let db = Builder::new_local("u06.db")
        .build()
        .await
        .expect("DB を開けません");
    let conn = db.connect().expect("接続できません");

    match migrate(&conn).await {
        Ok(applied) if applied.is_empty() => println!("新しく適用したもの: なし"),
        Ok(applied) => println!("新しく適用したもの: {applied:?}"),
        Err(error) => {
            // エラーの原因（source）をたどって全部表示する
            println!("エラー: {error}");
            let mut source = std::error::Error::source(&error);
            while let Some(cause) = source {
                println!("  原因: {cause}");
                source = cause.source();
            }
        }
    }
    let applied = applied_versions(&conn).await.expect("読めません");
    println!("適用済みの版: {applied:?}");
}
