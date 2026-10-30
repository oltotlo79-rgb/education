REPLACEMENTS = [("""        None => {
            verify_password(password, &DUMMY_HASH).await?;
            Ok(None)
        }""", """        // わざと間違えた版: 登録が無ければ、照合せずにすぐ返す
        None => Ok(None),""")]
