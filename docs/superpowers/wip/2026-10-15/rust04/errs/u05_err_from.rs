use thiserror::Error;

#[derive(Debug, Error)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

#[derive(Debug, Error)]
enum AppError {
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
    // EmailError を AppError に変換する方法が無い
    let email = parse_email(input)?;
    Ok(email)
}

fn main() {
    println!("{:?}", signup("alice@example.com"));
    println!("{:?}", AppError::NotFound);
}
