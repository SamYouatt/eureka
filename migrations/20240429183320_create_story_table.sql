-- Add story table
CREATE TABLE stories(
    id uuid NOT NULL PRIMARY KEY,
    story TEXT NOT NULL,
    idea_id UUID NOT NULL REFERENCES ideas (id),
    user_id UUID NOT NULL REFERENCES users (id),
    created_at timestamptz NOT NULL
)
