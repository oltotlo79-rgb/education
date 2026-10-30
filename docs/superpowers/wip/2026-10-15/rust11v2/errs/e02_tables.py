# 第11巻 UNIT 02 で、RustNote のコピーの src/repository/notes.rs に当てる書き換え。
# バックスラッシュを含むので、raw 文字列で書く。

# 例2-2: 検索の SQL から ESCAPE '\' を消す
NO_ESCAPE = [
    (
        r"""        let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";""",
        r"""        let condition = "user_id = ?1 AND (title LIKE ?2 OR body LIKE ?2)";""",
    )
]

# エラー1: 削除の SQL から、持ち主の条件を消す
NO_OWNER = [
    (
        '                "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",',
        '                "DELETE FROM notes WHERE id = ?1",',
    )
]
