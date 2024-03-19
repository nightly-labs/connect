CREATE TABLE ip_addresses (
    ip_addr TEXT NOT NULL UNIQUE,
    last_updated_at TIMESTAMPTZ NOT NULL,
    country TEXT,
    city TEXT,
    lat FLOAT8,
    lon FLOAT8
);

CREATE INDEX ip_addresses_last_updated_at_idx ON ip_addresses (last_updated_at);
