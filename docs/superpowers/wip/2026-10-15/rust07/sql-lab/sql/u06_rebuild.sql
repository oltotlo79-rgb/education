-- 本文に「10,000 文字まで」の CHECK を足したい（ALTER TABLE では後から CHECK を足せない）
BEGIN;
-- 1) 新しい形の表を、別の名前で作る
CREATE TABLE notes_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    body TEXT NOT NULL DEFAULT '' CHECK (length(body) <= 10000),
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)),
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1))
);
-- 2) 中身を写す
INSERT INTO notes_new (id, user_id, title, body, done, version, created_at, updated_at, pinned)
SELECT id, user_id, title, body, done, version, created_at, updated_at, pinned FROM notes;
-- 3) 古い表を消し、新しい表の名前を変える
DROP TABLE notes;
ALTER TABLE notes_new RENAME TO notes;
-- 4) 索引を作り直す（古い表と一緒に消えている）
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
COMMIT;
-- 5) 確かめる: 件数が同じで、新しい CHECK が効いている
SELECT COUNT(*) AS notes FROM notes;
UPDATE notes SET body = printf('%10001s', '') WHERE id = 1;
