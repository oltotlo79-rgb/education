CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
