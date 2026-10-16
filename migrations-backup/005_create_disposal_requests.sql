-- ============================================================
-- EcoPoint
-- Migration: 005_create_disposal_requests.sql
-- ============================================================

CREATE TABLE IF NOT EXISTS disposal_requests (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL,

    collection_point_id UUID NOT NULL,

    status VARCHAR(20) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL,

    updated_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT fk_disposal_request_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_disposal_request_collection_point
        FOREIGN KEY (collection_point_id)
        REFERENCES collection_points(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_disposal_requests_user
ON disposal_requests(user_id);

CREATE INDEX IF NOT EXISTS idx_disposal_requests_status
ON disposal_requests(status);

CREATE INDEX IF NOT EXISTS idx_disposal_requests_collection
ON disposal_requests(collection_point_id);