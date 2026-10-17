-- ① 同じ名前の索引をもう一度作る
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- ② 列名のつづり間違い
CREATE INDEX idx_notes_usr ON notes (usr_id);
-- ③ 列の順番を逆にした索引（updated_at が先）だけがあると……
DROP INDEX idx_notes_user_updated;
CREATE INDEX idx_notes_updated_user ON notes (updated_at DESC, user_id);
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ④ 存在しない索引を消す
DROP INDEX idx_nope;
