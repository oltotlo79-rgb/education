import { psql } from './h.mjs';

await psql([
  `DROP TABLE IF EXISTS practice_books`,
  `CREATE TABLE practice_books (
  id integer PRIMARY KEY,
  title text NOT NULL,
  author text NOT NULL,
  status text NOT NULL
)`,
  `INSERT INTO practice_books (id, title, author, status) VALUES
  (1, 'リーダブルコード', 'Boswell', 'done'),
  (2, 'プロを目指す人〜', '鈴木僚太', 'reading'),
  (3, '達人プログラマー', 'Thomas', 'unread')`,
  `SELECT * FROM practice_books ORDER BY id`,
  `INSERT INTO practice_books (id, title, author, status) VALUES (4, 'SQL入門', 'ミック', 'unread')`,
  `UPDATE practice_books SET status = 'done' WHERE id = 2`,
  `DELETE FROM practice_books WHERE id = 3`,
  `SELECT * FROM practice_books ORDER BY id`,
], { echo: true });
