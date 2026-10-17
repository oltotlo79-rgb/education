-- ① Bob のノートを新しい順に（5 と 8 は更新日時が同じ）
SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC;
-- ② 同じ日時なら id の大きい順、と決めておく（並びが必ず1通りに決まる）
SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC, id DESC;
-- ③ ページ送り: 1ページ 2 件で、Alice の 2 ページ目（3〜4 件目）
SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 2;
-- ④ LIKE で部分一致検索（タイトルか本文に「買い物」を含む）
SELECT id, user_id, title FROM notes WHERE title LIKE '%買い物%' OR body LIKE '%買い物%';
-- ⑤ 「%」を探したいのに、% は「何でもよい文字の並び」の意味になってしまう
SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%%%';
-- ⑥ ESCAPE で「\ の次の文字は、ただの文字」と決める
SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%\%%' ESCAPE '\';
