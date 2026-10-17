-- 索引の無い sessions 表を作る（練習用）
CREATE TABLE sessions (
    token_hash BLOB PRIMARY KEY,
    user_id INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);
-- 期限切れのセッションを消す文の計画（索引なし）
EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000;
-- expires_at の索引を作る
CREATE INDEX idx_sessions_expires ON sessions (expires_at);
-- もう一度、計画を見る
EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000;
