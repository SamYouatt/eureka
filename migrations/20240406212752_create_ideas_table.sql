-- Create ideas table
CREATE TABLE ideas(
    id uuid NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    tagline TEXT NOT NULL,
    description TEXT,
    created_at timestamptz NOT NULL
);
