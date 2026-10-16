-- ============================================================
-- EcoPoint
-- Migration: 009_fix_password_reset_token.sql
-- ============================================================

ALTER TABLE password_reset_tokens
ADD COLUMN IF NOT EXISTS token TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_password_reset_tokens_token
ON password_reset_tokens(token);

ALTER TABLE password_reset_tokens
ALTER COLUMN token SET NOT NULL;