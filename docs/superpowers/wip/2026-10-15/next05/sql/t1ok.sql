BEGIN;
SELECT id, title, status FROM practice_books WHERE id = 4;
UPDATE practice_books SET status = 'reading' WHERE id = 4;
SELECT * FROM practice_books ORDER BY id;
ROLLBACK;
