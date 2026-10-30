REPLACEMENTS = [
    ("        let deleted = conn\n            .execute(\n                \"DELETE FROM notes", "        let _deleted = conn\n            .execute(\n                \"DELETE FROM notes"),
    ("        if deleted == 0 {\n            return Err(RepoError::NotFound);\n        }\n", ""),
]
