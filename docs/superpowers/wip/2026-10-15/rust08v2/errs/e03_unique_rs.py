REPLACEMENTS = [("""    // 2) パスワードをハッシュ化して保存する（生のパスワードは保存しない）
""", """    // わざと間違えた版: 先に探して、無ければ登録する（DB の UNIQUE は外した）
    if app_state(cx).users.find_by_email(&email).await?.is_some() {
        form.email_error = Some(AppError::EmailTaken.to_string());
        return Ok(SignupOutcome::Rejected(StatusCode::CONFLICT, form));
    }

    // 2) パスワードをハッシュ化して保存する（生のパスワードは保存しない）
""")]
