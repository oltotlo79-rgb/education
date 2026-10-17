-- 索引を作る前の DB の大きさ（ページ数。1ページは page_size バイト）
PRAGMA page_size;
PRAGMA page_count;
-- 索引を作る
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- 作った後の大きさ
PRAGMA page_count;
