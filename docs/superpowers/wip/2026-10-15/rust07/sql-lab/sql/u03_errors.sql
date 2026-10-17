-- ① UPDATE でも CHECK は守られる: タイトルを空にする
UPDATE notes SET title = '' WHERE id = 1 AND user_id = 1;
-- ② NOT NULL: 本文を NULL にする
UPDATE notes SET body = NULL WHERE id = 1 AND user_id = 1;
-- ③ 列の数と値の数が合わない
INSERT INTO notes (user_id, title, body) VALUES (1, 'タイトルだけ');
-- ④ 存在しない会員に付け替える
UPDATE notes SET user_id = 99 WHERE id = 1 AND user_id = 1;
-- ⑤ WHERE を書き忘れた UPDATE（エラーにならず、全部の行が変わってしまう）
UPDATE notes SET done = 1;
SELECT done, COUNT(*) AS notes FROM notes GROUP BY done;
