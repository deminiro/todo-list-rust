-- This file should undo anything in `up.sql`
ALTER TABLE tasks
DROP COLUMN IF EXISTS completed;