REPLACEMENTS = [("""    // メールが無いのか、パスワードが違うのかは区別しない
    let error = Some(LOGIN_FAILED.to_string());
""", """    // わざと間違えた版: 登録が無いのか、パスワードが違うのかで、文を変える
    let registered = match crate::domain::Email::parse(&input.email) {
        Ok(email) => crate::state::app_state(cx)
            .users
            .find_by_email(&email)
            .await?
            .is_some(),
        Err(_) => false,
    };
    let error = Some(
        if registered {
            "パスワードが違います"
        } else {
            "そのメールアドレスは登録されていません"
        }
        .to_string(),
    );
""")]
