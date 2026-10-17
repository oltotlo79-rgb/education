-- ① JOIN: ノートと、その持ち主のメールアドレスを横に並べる
SELECT notes.id, users.email, notes.title
FROM notes
JOIN users ON users.id = notes.user_id
ORDER BY notes.id;
-- ② LEFT JOIN + GROUP BY: 会員ごとのノートの数（0 件の Carol も出す）
SELECT users.email, COUNT(notes.id) AS note_count
FROM users
LEFT JOIN notes ON notes.user_id = users.id
GROUP BY users.id
ORDER BY users.id;
