CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sequence_id BIGSERIAL,
    name VARCHAR NOT NULL,
    unit VARCHAR NOT NULL,
    stock DOUBLE PRECISION NOT NULL,
    rack VARCHAR,
    location VARCHAR,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE INDEX items_sequence_id_idx ON items (sequence_id);