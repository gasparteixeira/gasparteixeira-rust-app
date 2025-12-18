-- Migration: Add unique constraint to email column
-- Date: 2025-12-18
-- Description: Ensures email addresses are unique across all users

-- Add unique constraint to email column
-- Note: If you have duplicate emails, this will fail. Remove duplicates first.
ALTER TABLE users ADD CONSTRAINT users_email_unique UNIQUE (email);

-- Optional: To check for duplicate emails before running this migration:
-- SELECT email, COUNT(*) FROM users GROUP BY email HAVING COUNT(*) > 1;
