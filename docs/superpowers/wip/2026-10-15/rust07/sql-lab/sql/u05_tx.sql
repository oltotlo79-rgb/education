-- ① BEGIN → INSERT → ROLLBACK: 取り消すと、何も残らない
SELECT COUNT(*) AS before FROM notes;
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '取り消されるノート');
SELECT COUNT(*) AS during FROM notes;
ROLLBACK;
SELECT COUNT(*) AS after_rollback FROM notes;
-- ② BEGIN → INSERT → COMMIT: 確定すると残る
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '確定するノート');
COMMIT;
SELECT COUNT(*) AS after_commit FROM notes;
