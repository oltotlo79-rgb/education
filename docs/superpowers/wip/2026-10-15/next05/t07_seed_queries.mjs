import { psql } from './h.mjs';

await psql([
  `DELETE FROM books`,
  `INSERT INTO books (title, author, publisher, published_date, rating, status, notes) VALUES
('ノルウェイの森', '村上春樹', '講談社', '1987-09-04', 5, 'completed', '名作。'),
('人間失格', '太宰治', '新潮社', '1948-06-25', 4, 'completed', '代表作。'),
('こころ', '夏目漱石', '岩波書店', '1914-09-20', 5, 'reading', '読書中。'),
('銀河鉄道の夜', '宮沢賢治', '岩波書店', '1934-01-01', NULL, 'want_to_read', '気になる。'),
('コンビニ人間', '村田沙耶香', '文藝春秋', '2016-07-27', 4, 'completed', '一気に読了。')`,
], {});
await psql([
  `SELECT title, rating FROM books WHERE rating >= 4 ORDER BY rating DESC, title`,
  `SELECT COUNT(*) FROM books`,
  `SELECT status, COUNT(*) AS count FROM books GROUP BY status ORDER BY status`,
  `SELECT title, author FROM books WHERE status = 'reading' OR rating IS NULL ORDER BY title`,
  `SELECT title FROM books WHERE publisher = '岩波書店' ORDER BY published_date`,
], { echo: true });
