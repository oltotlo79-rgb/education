-- ① 記録を見ずに 0004 をもう一度流してしまう
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
-- ② 行がある表に、DEFAULT の無い NOT NULL の列を足す
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL;
-- ③ 直し方: DEFAULT を付ける（既にある行にはその値が入る）
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL DEFAULT 'none';
SELECT id, title, pinned, color FROM notes WHERE user_id = 2;
