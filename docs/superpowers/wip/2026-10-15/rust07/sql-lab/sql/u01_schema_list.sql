-- DB の中にある表と索引の一覧（sqlite_schema は SQLite が自動で持っている表）
SELECT type, name, tbl_name FROM sqlite_schema ORDER BY tbl_name, type DESC, name;
