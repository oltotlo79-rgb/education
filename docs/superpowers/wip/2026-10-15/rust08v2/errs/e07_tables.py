"""第8巻 UNIT 07 のわざと間違えた版の置き換え表（バックスラッシュを含むので raw 文字列で書く）。"""

# エラー1: 完了の切り替えの SQL から、持ち主の条件を外す
TOGGLE = [
    (
        "              WHERE id = ?1 AND user_id = ?2\n              RETURNING {NOTE_COLUMNS}\"\n        );\n        let mut rows = conn\n            .query(&sql, libsql::params![id.get(), owner.get()])",
        "              WHERE id = ?1\n              RETURNING {NOTE_COLUMNS}\"\n        );\n        let mut rows = conn\n            .query(&sql, libsql::params![id.get(), owner.get()])",
    )
]

# エラー2: 検索の条件から、かっこを外す
PRECEDENCE = [
    (
        r"""        let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";""",
        r"""        let condition = "user_id = ?1 AND title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\'";""",
    )
]

# エラー3: 持ち主を渡し忘れる
ARITY = [(".toggle_done(user.id, id)", ".toggle_done(id)")]
