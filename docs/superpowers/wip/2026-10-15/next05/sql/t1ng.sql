BEGIN;
UPDATE practice_books SET status = 'reading';
SELECT * FROM practice_books ORDER BY id;
ROLLBACK;
