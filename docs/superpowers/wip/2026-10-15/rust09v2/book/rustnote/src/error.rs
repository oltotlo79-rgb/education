//! エラーの分類。
//!
//! - `RepoError` … DB の層で起きる失敗（一意制約違反・version の食い違い・接続の失敗など）
//! - `AppError`  … 利用者への返事を決めるための分類。HTTP のステータスコードに対応する
//!
//! | AppError | ステータス | 起きる場面 |
//! |---|---|---|
//! | `InvalidTitle` など入力の検証エラー | 400 | 空のタイトル、形式の違うメール |
//! | `Unauthorized` | 401 | ログインしていない（通常はログイン画面へのリダイレクトで扱う） |
//! | `NotFound` | 404 | 存在しないノート、**他人のノート**（あること自体を隠す） |
//! | `EmailTaken` / `Conflict` | 409 | 登録済みのメール、ほかの画面で先に更新された |
//! | `Internal` | 500 | DB の故障など。画面には詳細を出さず、ログに原因を書く |

use thiserror::Error;
use topcoat::router::StatusCode;

use crate::domain::{BodyError, EmailError, PasswordError, TitleError};

/// いろいろな種類のエラーをまとめて入れる箱（Internal の中身に使う）
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

// ---------------------------------------------------------------------------
// DB の層のエラー
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("一意制約に違反しました: {0}")]
    UniqueViolation(String),
    #[error("version が一致しません（ほかの更新が先に保存されました）")]
    VersionMismatch,
    #[error("対象のデータが見つかりません")]
    NotFound,
    #[error("DB の値を読み取れません: {0}")]
    InvalidData(String),
    #[error("DB の操作に失敗しました")]
    Database(#[source] libsql::Error),
}

// libsql のエラーを RepoError に変換する。
// 「UNIQUE constraint failed」はローカル（SQLite）でも Turso でも同じ文言なので、文言で見分ける。
impl From<libsql::Error> for RepoError {
    fn from(error: libsql::Error) -> Self {
        let message = error.to_string();
        if message.contains("UNIQUE constraint failed") {
            RepoError::UniqueViolation(message)
        } else {
            RepoError::Database(error)
        }
    }
}

// ---------------------------------------------------------------------------
// アプリの層のエラー
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    InvalidTitle(#[from] TitleError),
    #[error(transparent)]
    InvalidBody(#[from] BodyError),
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),
    #[error(transparent)]
    InvalidPassword(#[from] PasswordError),
    #[error("ログインしてください")]
    Unauthorized,
    #[error("ページが見つかりません")]
    NotFound,
    #[error("そのメールアドレスは既に登録されています")]
    EmailTaken,
    #[error("ほかの画面で先に更新されました。読み込み直してください")]
    Conflict,
    #[error("サーバーの中でエラーが起きました")]
    Internal(#[source] BoxError),
}

// DB の失敗を、利用者への返事の分類に変換する
impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            other => AppError::Internal(Box::new(other)),
        }
    }
}

impl AppError {
    /// 想定外の失敗を Internal に包む
    pub fn internal(error: impl Into<BoxError>) -> AppError {
        AppError::Internal(error.into())
    }

    /// HTTP のステータスコード
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidTitle(_)
            | AppError::InvalidBody(_)
            | AppError::InvalidEmail(_)
            | AppError::InvalidPassword(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::EmailTaken | AppError::Conflict => StatusCode::CONFLICT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// 画面に出してよい文章（500 のときは中身を隠す）
    pub fn public_message(&self) -> String {
        match self {
            AppError::Internal(_) => "時間をおいてもう一度お試しください".to_string(),
            other => other.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_errors_map_to_app_errors() {
        assert!(matches!(
            AppError::from(RepoError::VersionMismatch),
            AppError::Conflict
        ));
        assert!(matches!(
            AppError::from(RepoError::NotFound),
            AppError::NotFound
        ));
        assert!(matches!(
            AppError::from(RepoError::InvalidData("x".to_string())),
            AppError::Internal(_)
        ));
    }

    #[test]
    fn status_codes_and_public_messages() {
        assert_eq!(
            AppError::from(TitleError::Empty).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(AppError::NotFound.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(AppError::Conflict.status_code(), StatusCode::CONFLICT);
        let internal = AppError::internal("disk full");
        assert_eq!(internal.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        // 500 のときは原因（disk full）を利用者に見せない
        assert!(!internal.public_message().contains("disk full"));
    }
}
