-- 0003: ログイン中のセッションのテーブル
-- token_hash は Topcoat が発行したトークンの SHA-256（32 バイト）。生のトークンは保存しない。
-- expires_at は有効期限。1970-01-01 からの秒数（UNIX 時刻）で持ち、比較しやすくする。
CREATE TABLE sessions (
    token_hash BLOB PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_sessions_user ON sessions (user_id);
CREATE INDEX idx_sessions_expires ON sessions (expires_at);
