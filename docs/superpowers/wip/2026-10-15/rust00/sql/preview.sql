CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE
);
CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    done INTEGER NOT NULL DEFAULT 0
);
INSERT INTO users (email) VALUES ('hanako@example.com');
INSERT INTO users (email) VALUES ('taro@example.com');
INSERT INTO notes (user_id, title) VALUES (1, '牛乳を買う');
INSERT INTO notes (user_id, title) VALUES (1, 'Rust の本を読む');
INSERT INTO notes (user_id, title) VALUES (2, '歯医者の予約');
.headers on
.mode list
.separator " | "
SELECT id, title, done FROM notes WHERE user_id = 1;
