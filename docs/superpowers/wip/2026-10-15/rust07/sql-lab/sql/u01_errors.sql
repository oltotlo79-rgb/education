-- ① UNIQUE 違反: 同じメールアドレスで2人目を登録する
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-x');
-- ② NOT NULL 違反: password_hash を書き忘れる
INSERT INTO users (email) VALUES ('carol@example.com');
-- ③ CHECK 違反: 空のタイトル
INSERT INTO notes (user_id, title) VALUES (1, '');
-- ④ CHECK 違反: done に 0 と 1 以外
INSERT INTO notes (user_id, title, done) VALUES (1, 'テスト', 2);
-- ⑤ FOREIGN KEY 違反: 存在しない会員（id = 99）のノート
INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート');
-- ⑥ 構文エラー: 列名の間のカンマを忘れた
INSERT INTO notes (user_id title) VALUES (1, 'カンマ忘れ');
-- 失敗した文は、1行も追加していない
SELECT (SELECT COUNT(*) FROM users) AS users_count, (SELECT COUNT(*) FROM notes) AS notes_count;
