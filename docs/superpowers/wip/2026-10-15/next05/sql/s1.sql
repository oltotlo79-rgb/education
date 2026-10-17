DROP TABLE IF EXISTS practice_books;
CREATE TABLE practice_books (
  id integer PRIMARY KEY,
  title text NOT NULL,
  author text NOT NULL,
  status text NOT NULL
);
INSERT INTO practice_books (id, title, author, status) VALUES
  (1, 'リーダブルコード', 'Boswell', 'completed'),
  (2, 'プロを目指す人〜', '鈴木僚太', 'reading'),
  (3, '達人プログラマー', 'Thomas', 'want_to_read');
INSERT INTO practice_books (id, title, author, status) VALUES (4, 'SQL入門', 'ミック', 'want_to_read');
UPDATE practice_books SET status = 'completed' WHERE id = 2;
DELETE FROM practice_books WHERE id = 3;
SELECT * FROM practice_books ORDER BY id;
