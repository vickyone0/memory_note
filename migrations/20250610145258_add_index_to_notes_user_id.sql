-- Add migration script here
CREATE INDEX idx_notes_user_id ON notes(user_id);