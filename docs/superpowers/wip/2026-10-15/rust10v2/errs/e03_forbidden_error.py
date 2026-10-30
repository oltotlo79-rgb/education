REPLACEMENTS = [
    ('    #[error("ページが見つかりません")]\n    NotFound,\n', '    #[error("ページが見つかりません")]\n    NotFound,\n    #[error("このノートを見る権限がありません")]\n    Forbidden,\n'),
    ('            AppError::NotFound => StatusCode::NOT_FOUND,\n', '            AppError::NotFound => StatusCode::NOT_FOUND,\n            AppError::Forbidden => StatusCode::FORBIDDEN,\n'),
]
