-- This file should undo anything in `up.sql`
ALTER TABLE tasks
DROP COLUMN IF EXISTS description,
DROP COLUMN IF EXISTS priority,
DROP COLUMN IF EXISTS completed,
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;
