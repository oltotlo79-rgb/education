//! 設定（Config）: 環境変数から読み込む。
//!
//! | 環境変数 | 意味 | 既定値 |
//! |---|---|---|
//! | `DATABASE_URL` | ローカルの DB ファイルのパス、または Turso の URL（`libsql://...`） | `rustnote.db` |
//! | `DATABASE_AUTH_TOKEN` | Turso の認証トークン（リモートのときだけ必要） | なし |
//! | `HOST` / `PORT` | 待ち受けるアドレス。`topcoat::start` が直接読む | `127.0.0.1` / `3000` |

use std::env;

use thiserror::Error;

/// DATABASE_URL が無いときに使うローカルの DB ファイル
pub const DEFAULT_DATABASE_URL: &str = "rustnote.db";

/// アプリの設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// ローカルファイルのパス（例: `rustnote.db`）か、Turso の URL（例: `libsql://...`）
    pub database_url: String,
    /// Turso の認証トークン。ローカルファイルのときは `None`
    pub database_auth_token: Option<String>,
}

/// 設定を読めなかった理由
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("環境変数 {0} の値が UTF-8 の文字列として読めません")]
    NotUnicode(&'static str),
    #[error("DATABASE_URL が Turso（{0}）なのに DATABASE_AUTH_TOKEN が設定されていません")]
    MissingAuthToken(String),
}

impl Config {
    /// 環境変数から設定を作る
    pub fn from_env() -> Result<Config, ConfigError> {
        let database_url =
            read_var("DATABASE_URL")?.unwrap_or_else(|| DEFAULT_DATABASE_URL.to_string());
        // 空文字の DATABASE_AUTH_TOKEN は「設定されていない」と同じに扱う
        let database_auth_token = read_var("DATABASE_AUTH_TOKEN")?.filter(|t| !t.is_empty());
        Config::new(database_url, database_auth_token)
    }

    /// 値を直接指定して設定を作る（テストでも使う）
    pub fn new(
        database_url: String,
        database_auth_token: Option<String>,
    ) -> Result<Config, ConfigError> {
        let config = Config {
            database_url,
            database_auth_token,
        };
        if config.requires_auth_token() && config.database_auth_token.is_none() {
            return Err(ConfigError::MissingAuthToken(config.database_url));
        }
        Ok(config)
    }

    /// ネットワークの向こうの DB（Turso / sqld）につなぐなら true
    pub fn is_remote(&self) -> bool {
        ["libsql://", "https://", "http://", "wss://", "ws://"]
            .iter()
            .any(|prefix| self.database_url.starts_with(prefix))
    }

    /// Turso（インターネット上の DB）ならトークンが必須
    fn requires_auth_token(&self) -> bool {
        ["libsql://", "https://", "wss://"]
            .iter()
            .any(|prefix| self.database_url.starts_with(prefix))
    }
}

/// 環境変数を1つ読む。無ければ `Ok(None)`
fn read_var(name: &'static str) -> Result<Option<String>, ConfigError> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::NotUnicode(name)),
    }
}

/// 起動時のログに出す「待ち受けアドレス」。
/// `topcoat::start` と同じ規則（HOST / PORT、既定 127.0.0.1:3000）で組み立てる。
pub fn listen_address() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    format!("{host}:{port}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_file_needs_no_token() {
        let config = Config::new("rustnote.db".to_string(), None).unwrap();
        assert!(!config.is_remote());
    }

    #[test]
    fn turso_url_needs_a_token() {
        let result = Config::new("libsql://example.turso.io".to_string(), None);
        assert_eq!(
            result,
            Err(ConfigError::MissingAuthToken(
                "libsql://example.turso.io".to_string()
            ))
        );

        let config = Config::new(
            "libsql://example.turso.io".to_string(),
            Some("token".to_string()),
        )
        .unwrap();
        assert!(config.is_remote());
    }

    #[test]
    fn local_sqld_over_http_needs_no_token() {
        let config = Config::new("http://127.0.0.1:8080".to_string(), None).unwrap();
        assert!(config.is_remote());
    }
}
