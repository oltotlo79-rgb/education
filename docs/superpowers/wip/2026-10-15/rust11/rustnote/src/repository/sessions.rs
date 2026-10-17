//! ログイン中のセッションの保存と検索。
//!
//! Topcoat はトークンを発行してクッキーで運ぶだけで、セッションの記録はアプリが持つ。
//! ここではトークンの SHA-256（TokenHash、32 バイト）と、ユーザー・有効期限を保存する。

use std::time::SystemTime;

use async_trait::async_trait;
use topcoat::session::TokenHash;

use super::unix_seconds;
use super::users::row_to_user;
use crate::db::Db;
use crate::domain::{User, UserId};
use crate::error::RepoError;

/// セッションの保存先に求める約束
#[async_trait]
pub trait SessionRepository {
    /// ログインしたときに記録する
    async fn create(
        &self,
        token_hash: &TokenHash,
        user_id: UserId,
        expires_at: SystemTime,
    ) -> Result<(), RepoError>;

    /// トークンのハッシュから、ログイン中のユーザーを探す（期限切れなら None）
    async fn find_user(
        &self,
        token_hash: &TokenHash,
        now: SystemTime,
    ) -> Result<Option<User>, RepoError>;

    /// ログアウトしたときに消す
    async fn delete(&self, token_hash: &TokenHash) -> Result<(), RepoError>;

    /// 期限切れのセッションをまとめて消し、消した数を返す
    async fn delete_expired(&self, now: SystemTime) -> Result<u64, RepoError>;
}

/// libSQL（Turso / ローカルファイル）に保存する実装
pub struct LibsqlSessionRepository {
    db: Db,
}

impl LibsqlSessionRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionRepository for LibsqlSessionRepository {
    async fn create(
        &self,
        token_hash: &TokenHash,
        user_id: UserId,
        expires_at: SystemTime,
    ) -> Result<(), RepoError> {
        let conn = self.db.connect().await?;
        conn.execute(
            "INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?1, ?2, ?3)",
            // TokenHash は [u8; 32] として読める。to_vec() で BLOB として渡す
            libsql::params![token_hash.to_vec(), user_id.get(), unix_seconds(expires_at)],
        )
        .await?;
        Ok(())
    }

    async fn find_user(
        &self,
        token_hash: &TokenHash,
        now: SystemTime,
    ) -> Result<Option<User>, RepoError> {
        let conn = self.db.connect().await?;
        let mut rows = conn
            .query(
                "SELECT u.id, u.email
                   FROM sessions s
                   JOIN users u ON u.id = s.user_id
                  WHERE s.token_hash = ?1 AND s.expires_at > ?2",
                libsql::params![token_hash.to_vec(), unix_seconds(now)],
            )
            .await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_user(&row)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, token_hash: &TokenHash) -> Result<(), RepoError> {
        let conn = self.db.connect().await?;
        conn.execute(
            "DELETE FROM sessions WHERE token_hash = ?1",
            [token_hash.to_vec()],
        )
        .await?;
        Ok(())
    }

    async fn delete_expired(&self, now: SystemTime) -> Result<u64, RepoError> {
        let conn = self.db.connect().await?;
        let deleted = conn
            .execute(
                "DELETE FROM sessions WHERE expires_at <= ?1",
                [unix_seconds(now)],
            )
            .await?;
        Ok(deleted)
    }
}
