-- Add sessions table
CREATE TABLE sessions (
    id uuid NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    session_id VARCHAR NOT NULL,
    expires_at timestamptz NOT NULL
);
