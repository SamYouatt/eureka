-- Add user column to ideas table
ALTER TABLE ideas ADD COLUMN user_id UUID NOT NULL REFERENCES users (id);
