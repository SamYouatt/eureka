-- Remove user id unique constraint on ideas
ALTER TABLE ideas DROP CONSTRAINT ideas_user_id_key;
