-- 未完了（done = 0）のノートだけを、新しい順に 20 件
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 AND done = 0 ORDER BY updated_at DESC, id DESC LIMIT 20;
