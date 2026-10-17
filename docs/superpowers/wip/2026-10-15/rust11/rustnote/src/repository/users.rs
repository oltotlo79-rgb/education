//! ユーザーの保存と検索。

use async_trait::async_trait;
use libsql::Row;

use super::invalid_data;
use crate::db::Db;
use crate::domain::{Email, User, UserId};
use crate::error::RepoError;

/// ログインの確認に使う、パスワードのハッシュ付きのユーザー
#[derive(Debug, Clone)]
pub struct UserRecord {
    pub user: User,
    /// Argon2id の PHC 文字列（$argon2id$v=19$...）
    pub password_hash: String,
}

/// ユーザーの保存先に求める約束
#[async_trait]
pub trait UserRepository {
    /// ユーザーを登録する。同じメールが既にあれば RepoError::UniqueViolation
    async fn create(&self, email: &Email, password_hash: &str) -> Result<User, RepoError>;

    /// メールアドレスでユーザーを探す（パスワードのハッシュも返す）
    async fn find_by_email(&self, email: &Email) -> Result<Option<UserRecord>, RepoError>;
}

/// libSQL（Turso / ローカルファイル）に保存する実装
pub struct LibsqlUserRepository {
    db: Db,
}

impl LibsqlUserRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for LibsqlUserRepository {
    async fn create(&self, email: &Email, password_hash: &str) -> Result<User, RepoError> {
        let conn = self.db.connect().await?;
        // RETURNING で、追加した行の id をその場で受け取る
        let mut rows = conn
            .query(
                "INSERT INTO users (email, password_hash) VALUES (?1, ?2) RETURNING id, email",
                libsql::params![email.as_str(), password_hash],
            )
            .await?;
        let row = rows
            .next()
            .await?
            .ok_or_else(|| RepoError::InvalidData("INSERT が行を返しませんでした".to_string()))?;
        row_to_user(&row)
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<UserRecord>, RepoError> {
        let conn = self.db.connect().await?;
        let mut rows = conn
            .query(
                "SELECT id, email, password_hash FROM users WHERE email = ?1",
                [email.as_str()],
            )
            .await?;
        match rows.next().await? {
            Some(row) => {
                let user = row_to_user(&row)?;
                let password_hash = row.get::<String>(2)?;
                Ok(Some(UserRecord {
                    user,
                    password_hash,
                }))
            }
            None => Ok(None),
        }
    }
}

/// 行（id, email, ...）を User に変える。sessions.rs からも使う
pub(super) fn row_to_user(row: &Row) -> Result<User, RepoError> {
    let id = UserId::new(row.get::<i64>(0)?).map_err(|e| invalid_data("users.id", e))?;
    let email = Email::parse(&row.get::<String>(1)?).map_err(|e| invalid_data("users.email", e))?;
    Ok(User { id, email })
}
