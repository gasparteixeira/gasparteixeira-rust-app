-- Migration: Add password column to users table
-- Date: 2025-12-18
-- Description: Adds a password field to the existing users table

-- Add password column with a default value for existing rows
ALTER TABLE users ADD COLUMN IF NOT EXISTS password TEXT NOT NULL DEFAULT 'changeme';

-- Remove the default constraint after adding the column
-- (optional - uncomment if you don't want future inserts to have a default)
-- ALTER TABLE users ALTER COLUMN password DROP DEFAULT;
