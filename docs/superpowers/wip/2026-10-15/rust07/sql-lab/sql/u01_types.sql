-- 値の種類（ストレージクラス）を typeof() で調べる
SELECT typeof(42) AS a, typeof(3.14) AS b, typeof('こんにちは') AS c, typeof(x'CAFE') AS d, typeof(NULL) AS e;
-- 文字数とバイト数
SELECT length('あいう') AS chars, length(CAST('あいう' AS BLOB)) AS bytes;
-- 普通の表は、列の型と違う値も（変換できなければ）そのまま入ってしまう
CREATE TABLE loose (n INTEGER);
INSERT INTO loose VALUES ('12'), ('abc');
SELECT n, typeof(n) FROM loose;
-- STRICT を付けた表は、型の違う値を断る
CREATE TABLE strict_t (n INTEGER) STRICT;
INSERT INTO strict_t VALUES ('12');
INSERT INTO strict_t VALUES ('abc');
SELECT n, typeof(n) FROM strict_t;
