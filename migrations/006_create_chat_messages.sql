-- ============================================================
-- EcoPoint
-- Migration: 006_create_chat_messages.sql
-- ============================================================

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,

    request_id UUID NOT NULL,

    sender_id UUID NOT NULL,

    message TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_chat_message_request
        FOREIGN KEY (request_id)
        REFERENCES disposal_requests(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_chat_message_sender
        FOREIGN KEY (sender_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_chat_messages_request
ON chat_messages(request_id);

CREATE INDEX IF NOT EXISTS idx_chat_messages_sender
ON chat_messages(sender_id);

CREATE INDEX IF NOT EXISTS idx_chat_messages_created_at
ON chat_messages(created_at);