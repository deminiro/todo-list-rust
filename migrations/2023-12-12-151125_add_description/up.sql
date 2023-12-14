-- Your SQL goes here
ALTER TABLE tasks
ADD COLUMN IF NOT EXISTS description TEXT;