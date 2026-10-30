REPLACEMENTS = [("if verify_password(password, &record.password_hash).await? {", "if verify_password(password, &record.password_hash)? {")]
