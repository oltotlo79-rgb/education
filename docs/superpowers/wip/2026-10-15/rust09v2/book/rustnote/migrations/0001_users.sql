-- 0001: ユーザーのテーブル
-- email は UNIQUE（同じメールアドレスで二重に登録できない）。
-- password_hash には Argon2id の PHC 文字列（$argon2id$v=19$...）を入れる。生のパスワードは保存しない。
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
