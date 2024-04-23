-- Create users table
CREATE TABLE users(
    id uuid NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    created_at timestamptz NOT NULL
);
