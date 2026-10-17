BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '1件目');
INSERT INTO notes (user_id, title) VALUES (1, '');
-- 失敗に気づいたので、COMMIT ではなく ROLLBACK する
ROLLBACK;
SELECT id, title FROM notes WHERE id > 8;
