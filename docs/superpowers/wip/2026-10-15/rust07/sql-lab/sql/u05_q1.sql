BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, 'A');
COMMIT;
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, 'B');
INSERT INTO notes (user_id, title) VALUES (1, 'C');
ROLLBACK;
SELECT COUNT(*) AS notes FROM notes;
