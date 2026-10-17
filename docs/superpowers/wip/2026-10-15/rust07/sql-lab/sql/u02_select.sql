-- ① 必要な列だけを読む
SELECT id, user_id, title FROM notes;
-- ② WHERE で行を絞る（Alice = user_id 1 のノートだけ）
SELECT id, title, done FROM notes WHERE user_id = 1;
-- ③ 条件を AND でつなぐ（Alice の、まだ終わっていないノート）
SELECT id, title FROM notes WHERE user_id = 1 AND done = 0;
-- ④ ORDER BY で並べ替え（更新日時の新しい順）
SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC;
-- ⑤ LIMIT で件数を制限（新しい順に 2 件だけ）
SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC LIMIT 2;
-- ⑥ 数える
SELECT COUNT(*) AS alice_notes FROM notes WHERE user_id = 1;
