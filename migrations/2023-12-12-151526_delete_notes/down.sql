-- This file should undo anything in `up.sql`
ALTER TABLE tasks
ADD COLUMN IF NOT EXISTS notes TEXT