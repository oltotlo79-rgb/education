-- ① 索引が無いとき: 会員 42 番のノートを新しい順に 20 件
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ② 主キー（id）で1件探すとき
EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE id = 42;
-- ③ 索引を作る（RustNote の migrations/0002 と同じ）
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- ④ ①と同じ問い合わせの計画を、もう一度見る
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ⑤ 結果は同じ（索引は速さだけを変え、答えは変えない）
SELECT id, title, updated_at FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 3;
