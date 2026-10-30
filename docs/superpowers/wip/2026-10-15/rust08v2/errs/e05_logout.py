REPLACEMENTS = [("""    if let Some(token_hash) = session::stop(cx).await.map_err(AppError::internal)? {
        app_state(cx).sessions.delete(&token_hash).await?;
    }
""", """    // わざと間違えた版: クッキーは消すが、DB の記録は消さない
    session::stop(cx).await.map_err(AppError::internal)?;
""")]
