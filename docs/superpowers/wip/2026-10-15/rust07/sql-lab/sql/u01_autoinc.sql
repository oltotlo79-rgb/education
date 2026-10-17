-- AUTOINCREMENT なしと、ありの表を作る
CREATE TABLE plain (id INTEGER PRIMARY KEY, v TEXT NOT NULL);
CREATE TABLE auto (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT NOT NULL);
-- どちらにも3行入れてから、いちばん大きい id（3）の行を消す
INSERT INTO plain (v) VALUES ('a'), ('b'), ('c');
INSERT INTO auto (v) VALUES ('a'), ('b'), ('c');
DELETE FROM plain WHERE id = 3;
DELETE FROM auto WHERE id = 3;
-- もう1行ずつ入れると、新しい id はいくつになるか
INSERT INTO plain (v) VALUES ('d');
INSERT INTO auto (v) VALUES ('d');
SELECT 'plain' AS tbl, id, v FROM plain WHERE v = 'd'
UNION ALL
SELECT 'auto', id, v FROM auto WHERE v = 'd';
-- AUTOINCREMENT の表は「これまでの最大の id」を sqlite_sequence に覚えている
SELECT name, seq FROM sqlite_sequence;
