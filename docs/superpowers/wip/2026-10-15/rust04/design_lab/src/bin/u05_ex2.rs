use thiserror::Error;

#[derive(Debug, Error)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

#[derive(Debug, Error)]
enum AppError {
    // #[from] で From<EmailError> for AppError が自動で作られる
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),
    #[error("ノートが見つかりません")]
    NotFound,
}

fn parse_email(input: &str) -> Result<String, EmailError> {
    if input.contains('@') {
        Ok(input.to_string())
    } else {
        Err(EmailError)
    }
}

fn signup(input: &str) -> Result<String, AppError> {
    let email = parse_email(input)?;
    Ok(email)
}

fn main() {
    println!("{:?}", signup("alice@example.com"));
    match signup("alice") {
        Ok(_) => println!("登録しました"),
        Err(e) => println!("{e} / {e:?}"),
    }
    println!("{}", AppError::NotFound);
}
