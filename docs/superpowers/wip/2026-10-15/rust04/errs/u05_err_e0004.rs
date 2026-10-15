use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("ログインしてください")]
    Unauthorized,
    #[error("ノートが見つかりません")]
    NotFound,
    #[error("ほかの画面で先に更新されました")]
    Conflict,
}

impl AppError {
    // 新しい種類を足すと、ここの match に書き足すまでコンパイルが通らない
    fn status_code(&self) -> u16 {
        match self {

            AppError::NotFound => 404,
            AppError::Conflict => 409,
        }
    }
}

fn main() {
    for e in [
        AppError::Unauthorized,
        AppError::NotFound,
        AppError::Conflict,
    ] {
        println!("{} {}", e.status_code(), e);
    }
}
