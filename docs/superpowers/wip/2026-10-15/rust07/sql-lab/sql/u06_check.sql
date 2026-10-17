-- 0005 の途中で作った tags の表は残っているか？
SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name;
-- 記録された版
SELECT version FROM schema_migrations ORDER BY version;
