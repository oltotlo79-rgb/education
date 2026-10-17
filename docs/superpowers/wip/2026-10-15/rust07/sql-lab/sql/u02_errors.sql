-- ① 列名のつづり間違い
SELECT id, titel FROM notes;
-- ② 表名のつづり間違い
SELECT * FROM note;
-- ③ JOIN した2つの表に同じ名前の列（id）がある
SELECT id, email, title FROM notes JOIN users ON users.id = notes.user_id;
-- ④ NULL は = では比べられない（エラーにはならず、何も返らない）
SELECT NULL = NULL AS eq, NULL IS NULL AS is_null, 1 = NULL AS one_eq_null;
-- ⑤ 文字列を " で囲んでしまう（" は本来「列名」の印）
SELECT id, title FROM notes WHERE title = "title";
SELECT id, title FROM notes WHERE title = "買い物";
