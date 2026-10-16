-- ============================================================
-- EcoPoint
-- Migration: 008_create_password_reset_tokens.sql
-- ============================================================

CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL,

    token TEXT NOT NULL UNIQUE,

    expires_at TIMESTAMPTZ NOT NULL,

    used BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_password_reset_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_password_reset_token
ON password_reset_tokens(token);

CREATE INDEX IF NOT EXISTS idx_password_reset_user
ON password_reset_tokens(user_id);

CREATE INDEX IF NOT EXISTS idx_password_reset_expires
ON password_reset_tokens(expires_at);