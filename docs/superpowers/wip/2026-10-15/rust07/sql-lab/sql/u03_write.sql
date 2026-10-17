-- ① INSERT: Alice（1）のノートを追加し、RETURNING で入った行を返してもらう
INSERT INTO notes (user_id, title, body) VALUES (1, '牛乳を買う', '帰りにスーパーへ')
RETURNING id, title, done, version;
-- ② UPDATE: ノート 9 のタイトルを変え、version を 1 増やす
UPDATE notes SET title = '牛乳と卵を買う', version = version + 1
WHERE id = 9 AND user_id = 1;
SELECT id, title, done, version FROM notes WHERE id = 9;
-- ③ 完了の切り替え: done を 0 ⇔ 1 に反転する
UPDATE notes SET done = 1 - done, version = version + 1
WHERE id = 9 AND user_id = 1;
SELECT id, title, done, version FROM notes WHERE id = 9;
-- ④ DELETE: ノート 9 を消す
DELETE FROM notes WHERE id = 9 AND user_id = 1;
SELECT COUNT(*) AS found FROM notes WHERE id = 9;
