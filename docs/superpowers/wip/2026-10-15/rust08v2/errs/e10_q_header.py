# 問3: 守りのテストから、Sec-Fetch-Site のヘッダーを外す（tests/u10_guards.rs）
REPLACEMENTS = [('        .header("sec-fetch-site", "cross-site")\n', "")]
