-- ============================================================
-- EcoPoint
-- Migration: 010_add_used_to_password_reset_tokens.sql
-- ============================================================

ALTER TABLE password_reset_tokens
ADD COLUMN IF NOT EXISTS used BOOLEAN NOT NULL DEFAULT FALSE;