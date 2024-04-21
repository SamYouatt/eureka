-- Add unique constraint on user id in sessions table
ALTER TABLE sessions ADD CONSTRAINT unique_user_session UNIQUE (user_id);
