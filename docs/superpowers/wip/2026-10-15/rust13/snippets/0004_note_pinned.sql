-- 0004: ノートを一覧の上に固定する印（ピン留め）
-- 0（固定しない）か 1（固定する）。今あるノートは 0 になる。
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
