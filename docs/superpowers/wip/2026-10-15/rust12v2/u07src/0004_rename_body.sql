-- 0004: 本文の列の名前を body から content に変える（古い版のコードは body を探して失敗する）
ALTER TABLE notes RENAME COLUMN body TO content;
