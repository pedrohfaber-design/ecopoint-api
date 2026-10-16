CREATE TABLE IF NOT EXISTS collection_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name VARCHAR(100) NOT NULL,

    description TEXT NOT NULL,

    address TEXT NOT NULL,

    city VARCHAR(100) NOT NULL,

    state CHAR(2) NOT NULL,

    latitude DOUBLE PRECISION NOT NULL,

    longitude DOUBLE PRECISION NOT NULL,

    phone VARCHAR(20),

    email VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_collection_city
ON collection_points(city);

CREATE INDEX IF NOT EXISTS idx_collection_state
ON collection_points(state);