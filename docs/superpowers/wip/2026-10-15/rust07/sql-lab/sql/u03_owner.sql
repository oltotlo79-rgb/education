-- Alice（user_id = 1）が、URL の id を 3 に書き換えて Bob のノートを狙う
-- NG: id だけで探すと、他人のノートでも書き換えられてしまう
UPDATE notes SET title = '乗っ取り' WHERE id = 3;
SELECT id, user_id, title FROM notes WHERE id = 3;
-- （元に戻しておく）
UPDATE notes SET title = '旅行の計画' WHERE id = 3;
-- OK: 「id が 3」かつ「持ち主がログイン中の人（1）」の行だけを対象にする
UPDATE notes SET title = '乗っ取り' WHERE id = 3 AND user_id = 1;
DELETE FROM notes WHERE id = 3 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 3 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 3;
