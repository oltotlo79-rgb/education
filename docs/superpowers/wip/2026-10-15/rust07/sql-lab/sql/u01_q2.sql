-- (a) 本文に NULL をはっきり入れる
INSERT INTO notes (user_id, title, body) VALUES (2, 'メモ', NULL);
-- (b) タイトルが空白3つだけ
INSERT INTO notes (user_id, title) VALUES (1, '   ');
-- (c) 会員 3 のノート
INSERT INTO notes (user_id, title) VALUES (3, '宿題');
-- (d) done に 1
INSERT INTO notes (user_id, title, done) VALUES (2, '洗濯', 1);
SELECT id, user_id, '[' || title || ']' AS title, done FROM notes;
