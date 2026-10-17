-- ① 索引の先頭の列（user_id）を条件に使わないと、索引で探せない
EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE updated_at >= '2026-03-01';
-- ② LIKE '%…%' は索引で探せないが、user_id で先に絞れば残りを調べるだけで済む
EXPLAIN QUERY PLAN SELECT id FROM notes WHERE user_id = 42 AND title LIKE '%買い物%';
-- ③ UNIQUE 制約が自動で作った索引（sqlite_autoindex_users_1）が使われる
EXPLAIN QUERY PLAN SELECT id FROM users WHERE email = 'user42@example.com';
-- ④ 列に関数をかぶせると、その索引は使えない
EXPLAIN QUERY PLAN SELECT id FROM users WHERE lower(email) = 'user42@example.com';
-- ⑤ 数える問い合わせも索引で速くなる
EXPLAIN QUERY PLAN SELECT COUNT(*) FROM notes WHERE user_id = 42;
