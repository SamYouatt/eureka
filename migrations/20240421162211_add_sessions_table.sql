-- Add sessions table
CREATE TABLE sessions (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id),
    session_id VARCHAR NOT NULL,
    expires_at timestamptz NOT NULL
);
