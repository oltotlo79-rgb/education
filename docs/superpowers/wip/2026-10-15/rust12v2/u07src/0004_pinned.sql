-- 0004: ノートに「ピン留め」の列を足す（足すだけ。古い版のコードは、この列を知らなくても動く）
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
