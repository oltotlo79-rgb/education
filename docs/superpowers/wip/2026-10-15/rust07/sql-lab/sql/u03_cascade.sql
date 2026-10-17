-- 会員ごとのノートの数
SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id;
-- Bob（2）が退会する
DELETE FROM users WHERE id = 2;
-- ON DELETE CASCADE なので、Bob のノートも一緒に消えている
SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id;
