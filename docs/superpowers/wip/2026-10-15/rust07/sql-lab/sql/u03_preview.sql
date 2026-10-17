-- 消す前に、同じ WHERE で SELECT して「何行が対象か」を確かめる
SELECT id, title FROM notes WHERE user_id = 2 AND done = 1;
DELETE FROM notes WHERE user_id = 2 AND done = 1;
SELECT id, title, done FROM notes WHERE user_id = 2;
