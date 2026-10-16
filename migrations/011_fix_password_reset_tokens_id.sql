-- ============================================================
-- EcoPoint
-- Migration: 011_fix_password_reset_token_hash.sql
-- ============================================================

ALTER TABLE password_reset_tokens
DROP COLUMN IF EXISTS token_hash;

ALTER TABLE password_reset_tokens
ALTER COLUMN token SET NOT NULL;

ALTER TABLE password_reset_tokens
ALTER COLUMN used SET DEFAULT FALSE;