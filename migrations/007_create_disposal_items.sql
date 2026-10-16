-- ============================================================
-- EcoPoint
-- Migration: 007_create_disposal_items.sql
-- ============================================================

CREATE TABLE IF NOT EXISTS disposal_items (
    id UUID PRIMARY KEY,

    request_id UUID NOT NULL,

    waste_type_id UUID NOT NULL,

    quantity INTEGER NOT NULL,

    CONSTRAINT fk_disposal_item_request
        FOREIGN KEY (request_id)
        REFERENCES disposal_requests(id)
        ON DELETE CASCADE
);