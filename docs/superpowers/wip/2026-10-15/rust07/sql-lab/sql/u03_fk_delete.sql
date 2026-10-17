-- ON DELETE の指定が無い表で、ノートを持っている会員を消そうとする
DELETE FROM users WHERE id = 1;
