-- ① LEFT JOIN で、ノートが1件もない会員を探す（相手がいない行は NULL で埋まる）
SELECT users.id, users.email, notes.id AS note_id
FROM users
LEFT JOIN notes ON notes.user_id = users.id
WHERE notes.id IS NULL;
-- ② LIKE は英字の大文字と小文字を区別しない
SELECT 'Rust' LIKE 'rust' AS ascii_like, 'Rust' = 'rust' AS ascii_eq;
-- ③ 小文字の「rust」で検索しても、タイトルの「Rust」が見つかる
SELECT id, title FROM notes WHERE title LIKE '%rust%';
