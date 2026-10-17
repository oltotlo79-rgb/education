-- 会員（ユーザー）の表
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ノートの表。user_id で「どの会員のノートか」を指す
CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    body TEXT NOT NULL DEFAULT '',
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1))
);

-- 行を入れてみる（id は書かなくても 1, 2, … と自動で決まる）
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a');
INSERT INTO users (email, password_hash) VALUES ('bob@example.com', 'dummy-b');
INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト');
INSERT INTO notes (user_id, title, body) VALUES (2, '旅行の計画', '京都で紅葉を見る');

-- 中身を確かめる
SELECT id, email, password_hash FROM users;
SELECT * FROM notes;
