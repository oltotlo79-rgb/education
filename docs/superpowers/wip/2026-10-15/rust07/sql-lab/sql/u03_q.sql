-- 解答1: Bob（2）がノート 8 を完了にする（持ち主の条件つき）
UPDATE notes SET done = 1, version = version + 1, updated_at = datetime('now')
WHERE id = 8 AND user_id = 2
RETURNING id, title, done, version;
-- 解答2: Alice（1）が Bob のノート 5 を消そうとする
DELETE FROM notes WHERE id = 5 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 5;
-- 解答3: Alice が本文を書き換え、version と updated_at も更新して結果を受け取る
UPDATE notes SET body = '牛乳、卵、パン、バター', version = version + 1, updated_at = datetime('now')
WHERE id = 1 AND user_id = 1
RETURNING id, body, version, updated_at;
