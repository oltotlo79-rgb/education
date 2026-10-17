-- アプリを使い始めた（会員とノートができた）
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a');
INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト');
SELECT id, title, done FROM notes;
