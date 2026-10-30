# 第11巻 UNIT 03 エラー3: 検索の模様を「前に一致」にしてしまう（src/repository/notes.rs）
REPLACEMENTS = [('            Some(keyword) => format!("%{}%", escape_like(keyword)),', '            Some(keyword) => format!("{}%", escape_like(keyword)),')]
