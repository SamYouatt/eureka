-- Add unique constraint on user email
ALTER TABLE users ADD CONSTRAINT unique_emails UNIQUE (email);
