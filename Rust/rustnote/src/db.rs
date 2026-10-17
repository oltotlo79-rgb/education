//! DB（Turso / libSQL）への接続と、マイグレーション（テーブルの作成・変更）。
//!
//! - 開発ではローカルファイル（例: rustnote.db）を使う
//! - 本番では Turso の URL（libsql://...）と認証トークンを環境変数で渡す
//!
//! 接続（Connection）は処理のたびに `Db::connect` で作る。
//! ローカルファイルなら開くだけ、リモートなら HTTP の準備だけなので軽い。

use std::path::Path;
use std::sync::Arc;

use libsql::{Builder, Connection, Database, Transaction};
use thiserror::Error;

use crate::config::Config;

/// DB への入口。中身は Arc なので、clone しても同じ DB を指す（安く複製できる）
#[derive(Clone)]
pub struct Db {
    database: Arc<Database>,
    is_local: bool,
}

impl Db {
    /// 設定に従って、ローカルファイルかリモート（Turso）の DB を開く
    pub async fn open(config: &Config) -> Result<Db, libsql::Error> {
        if config.is_remote() {
            let token = config.database_auth_token.clone().unwrap_or_default();
            let database = Builder::new_remote(config.database_url.clone(), token)
                .build()
                .await?;
            Ok(Db {
                database: Arc::new(database),
                is_local: false,
            })
        } else {
            Db::open_local(&config.database_url).await
        }
    }

    /// ローカルファイルの DB を開く（無ければ作られる）
    pub async fn open_local(path: impl AsRef<Path>) -> Result<Db, libsql::Error> {
        let database = Builder::new_local(path).build().await?;
        let db = Db {
            database: Arc::new(database),
            is_local: true,
        };
        // WAL モード: 読み込みと書き込みが同時に来ても待たされにくくなる（ファイルに記録される）
        let conn = db.connect().await?;
        conn.execute_batch("PRAGMA journal_mode = WAL;").await?;
        Ok(db)
    }

    /// 新しい接続を作る
    pub async fn connect(&self) -> Result<Connection, libsql::Error> {
        let conn = self.database.connect()?;
        if self.is_local {
            // SQLite は接続ごとに設定が必要:
            // foreign_keys … REFERENCES（外部キー）を守らせる
            // busy_timeout … ほかの接続が書き込み中なら最大 5 秒待つ
            conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;")
                .await?;
        }
        Ok(conn)
    }

    /// DB に問い合わせができるか確かめる（GET /health で使う）
    pub async fn ping(&self) -> Result<(), libsql::Error> {
        let conn = self.connect().await?;
        let mut rows = conn.query("SELECT 1", ()).await?;
        rows.next().await?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// マイグレーション
// ---------------------------------------------------------------------------

/// 1つのマイグレーション（版の名前と SQL）
pub struct Migration {
    pub version: &'static str,
    pub sql: &'static str,
}

/// アプリが持つマイグレーションの一覧（古い順）。
/// SQL ファイルは include_str! でコンパイル時に実行ファイルへ埋め込む。
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: "0001_users",
        sql: include_str!("../migrations/0001_users.sql"),
    },
    Migration {
        version: "0002_notes",
        sql: include_str!("../migrations/0002_notes.sql"),
    },
    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
];

#[derive(Debug, Error)]
pub enum MigrateError {
    #[error("schema_migrations を準備できません")]
    Prepare(#[source] libsql::Error),
    #[error("マイグレーション {version} に失敗しました")]
    Apply {
        version: &'static str,
        #[source]
        source: libsql::Error,
    },
}

/// まだ適用していないマイグレーションだけを、1つずつトランザクションの中で適用する。
/// 戻り値は、今回新しく適用した版の名前の一覧。
pub async fn migrate(db: &Db) -> Result<Vec<&'static str>, MigrateError> {
    let conn = db.connect().await.map_err(MigrateError::Prepare)?;

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

    let applied = applied_versions(&conn)
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

/// schema_migrations に記録された版の名前を読む
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

/// 1つのマイグレーションの SQL を実行し、適用済みとして記録する
async fn apply(tx: &Transaction, migration: &Migration) -> Result<(), libsql::Error> {
    tx.execute_batch(migration.sql).await?;
    tx.execute(
        "INSERT INTO schema_migrations (version) VALUES (?1)",
        [migration.version],
    )
    .await?;
    Ok(())
}
