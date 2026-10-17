DELETE FROM books;
INSERT INTO books (title, author, publisher, published_date, rating, status, memo) VALUES
('ノルウェイの森', '村上春樹', '講談社', '1987-09-04', 5, 'completed', '名作。'),
('人間失格', '太宰治', '新潮社', '1948-06-25', 4, 'completed', '代表作。'),
('こころ', '夏目漱石', '岩波書店', '1914-09-20', 5, 'reading', '読書中。'),
('銀河鉄道の夜', '宮沢賢治', '岩波書店', '1934-01-01', NULL, 'want_to_read', '気になる。'),
('コンビニ人間', '村田沙耶香', '文藝春秋', '2016-07-27', 4, 'completed', '一気に読了。');
