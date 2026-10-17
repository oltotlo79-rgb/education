-- 2件まとめて保存したい。2件目が CHECK 違反になると……
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '1件目');
INSERT INTO notes (user_id, title) VALUES (1, '');
-- うっかり COMMIT すると、1件目だけが確定してしまう
COMMIT;
SELECT id, title FROM notes WHERE id > 8;
