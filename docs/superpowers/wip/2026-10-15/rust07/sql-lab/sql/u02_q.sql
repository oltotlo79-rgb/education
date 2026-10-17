-- 解答1: Alice（1）の完了済み（done = 1）のノートの数
SELECT COUNT(*) AS done_notes FROM notes WHERE user_id = 1 AND done = 1;
-- 解答2: Bob（2）のノートを、作った順（古い順）に
SELECT id, title, created_at FROM notes WHERE user_id = 2 ORDER BY created_at ASC, id ASC;
-- 解答3: Alice の 3 ページ目（1 ページ 2 件）
SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 4;
