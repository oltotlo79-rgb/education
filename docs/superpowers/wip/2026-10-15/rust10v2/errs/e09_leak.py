REPLACEMENTS = [("""        log_if_internal(app_error.status_code(), &error);
        (app_error.status_code(), app_error.public_message())""", """        log_if_internal(app_error.status_code(), &error);
        // わざと間違えた版: 原因の鎖を、そのまま画面に出す
        let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();
        (app_error.status_code(), causes.join(" <- "))""")]
