-- 外部キーの確認を切ってしまうと……
PRAGMA foreign_keys = OFF;
INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート');
SELECT id, user_id, title FROM notes;
-- 「指している先が無い行」を探す
PRAGMA foreign_key_check;
-- 元に戻す（この接続では、ここから先また確認される）
PRAGMA foreign_keys = ON;
PRAGMA foreign_keys;
