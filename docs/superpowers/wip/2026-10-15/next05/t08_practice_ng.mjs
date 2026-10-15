import { psql } from './h.mjs';

const reset = [
  `DROP TABLE IF EXISTS practice_books`,
  `CREATE TABLE practice_books (id integer PRIMARY KEY, title text NOT NULL, author text NOT NULL, status text NOT NULL)`,
  `INSERT INTO practice_books (id, title, author, status) VALUES
  (1, 'リーダブルコード', 'Boswell', 'done'),
  (2, 'プロを目指す人〜', '鈴木僚太', 'reading'),
  (4, 'SQL入門', 'ミック', 'unread')`,
];
await psql(reset);
console.log('==== NG ====');
await psql([`UPDATE practice_books SET status = 'done'`, `SELECT id, title, status FROM practice_books ORDER BY id`], { echo: true });
await psql(reset);
console.log('==== OK (select first) ====');
await psql([
  `SELECT id, title, status FROM practice_books WHERE id = 2`,
  `UPDATE practice_books SET status = 'done' WHERE id = 2`,
  `SELECT id, title, status FROM practice_books ORDER BY id`,
], { echo: true });
console.log('==== quotes ====');
await psql([
  `SELECT * FROM practice_books WHERE status = "reading"`,
  `SELECT * FROM practice_books WHERE status = 'reading'`,
], { echo: true });
await psql([`SELECT * FROM practice_books WHERE status = 'done'`], { echo: true });
