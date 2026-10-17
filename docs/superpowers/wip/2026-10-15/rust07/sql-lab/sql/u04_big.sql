-- 会員を 1,000 人作る（WITH RECURSIVE で 1〜1000 の数を作り、それを使って INSERT する）
WITH RECURSIVE seq(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM seq WHERE n < 1000)
INSERT INTO users (id, email, password_hash)
SELECT n, 'user' || n || '@example.com', 'dummy' FROM seq;

-- ノートを 100,000 件作る（1人あたり 100 件。更新日時は1分ずつずらす）
WITH RECURSIVE seq(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM seq WHERE n < 100000)
INSERT INTO notes (user_id, title, body, updated_at)
SELECT (n % 1000) + 1, 'ノート ' || n, '本文 ' || n, datetime('2026-01-01', '+' || n || ' minutes')
FROM seq;
