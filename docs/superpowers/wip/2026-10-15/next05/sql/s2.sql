INSERT INTO books (title, author, rating) VALUES ('テスト', 'テスト', 10);
INSERT INTO books (title, author, status) VALUES ('テスト', 'テスト', 'done');
INSERT INTO books (title) VALUES ('著者なし');
INSERT INTO books (title, author) VALUES ('こころ', '夏目漱石');
SELECT title, author, status, rating FROM books;
DELETE FROM books WHERE title = 'こころ';
