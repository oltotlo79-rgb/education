-- タブ A: 開いたときの version 1 で保存
UPDATE notes SET title = '買い物リスト（A）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1;
-- タブ B: 古い version 1 のまま保存
UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1;
-- タブ B: 画面を読み直して、最新の version 2 で保存し直す
UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 2;
SELECT id, title, version FROM notes WHERE id = 1;
