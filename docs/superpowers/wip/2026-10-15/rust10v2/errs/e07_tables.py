"""第10巻 UNIT 07 のわざと間違えた版の置き換え表（バックスラッシュを含むので、ここに raw 文字列で書く）。
edit_src.py からは、e07_no_escape.py などが、この表を読む。
"""

CONDITION = r"""        let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";"""

# エラー1: escape_like を通さない
NO_ESCAPE = [
    (
        """            Some(keyword) => format!("%{}%", escape_like(keyword)),""",
        """            Some(keyword) => format!("%{keyword}%"),""",
    )
]

# エラー2: ESCAPE '\' を外す（escape_like は残す）
NO_ESCAPE_CLAUSE = [
    (
        CONDITION,
        """        let condition = "user_id = ?1 AND (title LIKE ?2 OR body LIKE ?2)";""",
    )
]

# エラー3: キーワードを SQL の文字列に直接入れる
INJECTION = [
    (
        CONDITION,
        r"""        let condition = format!(
            "user_id = ?1 AND (title LIKE '{pattern}' ESCAPE '\\' OR body LIKE '{pattern}' ESCAPE '\\')"
        );""",
    )
]
