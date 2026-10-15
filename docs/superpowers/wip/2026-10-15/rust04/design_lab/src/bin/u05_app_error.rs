use thiserror::Error;

// ---- ドメインの層: 入力の検査で起きる失敗 ----
#[derive(Debug, Error, PartialEq, Eq)]
enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

// ---- 保存先（DB）の層: 保存で起きる失敗 ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("一意制約に違反しました（列: {0}）")]
    UniqueViolation(String),
    #[error("version が一致しません")]
    VersionMismatch,
    #[error("DB に接続できません")]
    Connection(#[source] std::io::Error),
}

// ---- アプリの層: 利用者への返事を決めるための分類 ----
#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    InvalidTitle(#[from] TitleError),
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),
    #[error("ノートが見つかりません")]
    NotFound,
    #[error("そのメールアドレスは既に登録されています")]
    EmailTaken,
    #[error("ほかの画面で先に更新されました。読み込み直してください")]
    Conflict,
    #[error("保存先でエラーが起きました")]
    Internal(#[source] RepoError),
}

// 保存先の失敗を、アプリの分類へ変換する
impl From<RepoError> for AppError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::VersionMismatch => AppError::Conflict,
            other => AppError::Internal(other),
        }
    }
}

impl AppError {
    // HTTP のステータスコード
    fn status_code(&self) -> u16 {
        match self {
            AppError::InvalidTitle(_) | AppError::InvalidEmail(_) => 400,
            AppError::NotFound => 404,
            AppError::EmailTaken | AppError::Conflict => 409,
            AppError::Internal(_) => 500,
        }
    }

    // 画面に出してよい文章（500 のときは中身を隠す）
    fn public_message(&self) -> String {
        match self {
            AppError::Internal(_) => "時間をおいてもう一度お試しください".to_string(),
            other => other.to_string(),
        }
    }
}

fn parse_title(input: &str) -> Result<String, TitleError> {
    let len = input.trim().chars().count();
    if len == 0 {
        return Err(TitleError::Empty);
    }
    if len > 80 {
        return Err(TitleError::TooLong { len, max: 80 });
    }
    Ok(input.trim().to_string())
}

fn parse_email(input: &str) -> Result<String, EmailError> {
    let email = input.trim().to_lowercase();
    match email.split_once('@') {
        Some((local, domain)) if !local.is_empty() && domain.contains('.') => Ok(email),
        _ => Err(EmailError),
    }
}

// ユースケース1: ノートのタイトルを変える（db は保存の結果のつもり）
fn rename_note(input: &str, found: bool, db: Result<(), RepoError>) -> Result<String, AppError> {
    // TitleError は #[from] のおかげで ? だけで AppError になる
    let title = parse_title(input)?;
    if !found {
        return Err(AppError::NotFound);
    }
    // RepoError は impl From のおかげで ? だけで AppError になる
    db?;
    Ok(title)
}

// ユースケース2: 登録する。一意制約違反だけは「登録済み」と言い換える
fn signup(input: &str, db: Result<(), RepoError>) -> Result<String, AppError> {
    let email = parse_email(input)?;
    db.map_err(|e| match e {
        RepoError::UniqueViolation(_) => AppError::EmailTaken,
        other => AppError::from(other),
    })?;
    Ok(email)
}

// HTTP の層の代わり: 結果を「画面に返すもの」と「ログに書くもの」に分ける
fn respond(result: Result<String, AppError>) {
    match result {
        Ok(value) => println!("200 OK: {value}"),
        Err(e) => {
            println!("{} 画面: {}", e.status_code(), e.public_message());
            // ログには原因までたどって書く
            let mut cause = std::error::Error::source(&e);
            while let Some(c) = cause {
                println!("    ログ（原因）: {c}");
                cause = c.source();
            }
        }
    }
}

fn main() {
    let down = || {
        std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "接続を拒否されました",
        )
    };

    respond(rename_note("牛乳を買う", true, Ok(())));
    respond(rename_note("   ", true, Ok(())));
    respond(rename_note("牛乳を買う", false, Ok(())));
    respond(rename_note(
        "牛乳を買う",
        true,
        Err(RepoError::VersionMismatch),
    ));
    respond(rename_note(
        "牛乳を買う",
        true,
        Err(RepoError::Connection(down())),
    ));
    respond(signup("alice@example", Ok(())));
    respond(signup(
        "alice@example.com",
        Err(RepoError::UniqueViolation("email".to_string())),
    ));
}
