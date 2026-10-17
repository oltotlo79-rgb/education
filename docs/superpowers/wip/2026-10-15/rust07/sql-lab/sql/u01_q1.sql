-- 解答1: タグの表
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 20),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (user_id, name)
);
-- 確かめる
INSERT INTO tags (user_id, name) VALUES (1, '仕事');
INSERT INTO tags (user_id, name) VALUES (2, '仕事');
INSERT INTO tags (user_id, name) VALUES (1, '仕事');
INSERT INTO tags (user_id, name) VALUES (1, '');
SELECT id, user_id, name FROM tags;
