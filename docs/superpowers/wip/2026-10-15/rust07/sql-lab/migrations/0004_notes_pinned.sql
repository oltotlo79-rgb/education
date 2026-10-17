-- 0004: ノートを「ピン留め」できるようにする
-- 既にある行には DEFAULT の 0 が入る
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
