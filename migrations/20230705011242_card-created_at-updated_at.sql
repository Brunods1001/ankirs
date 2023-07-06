-- Truncate card table
DELETE FROM card;

-- Add created_at and updated_at columns to card table
ALTER TABLE card ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
