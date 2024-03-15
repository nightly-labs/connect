CREATE TYPE subscription AS (
    subscription_type TEXT,
    valid_from BIGINT,
    valid_till BIGINT
);

CREATE TYPE request_status_enum AS ENUM (
    'Pending',
    'Completed',
    'Rejected',
    'TimedOut'
);

CREATE TYPE privilege_level_enum AS ENUM ('Read', 'Edit', 'Admin');

CREATE TYPE entity_type_enum AS ENUM ('App', 'Client');

CREATE TYPE session_type_enum AS ENUM ('Extension', 'Relay');

CREATE TYPE client_data AS (
    client_profile_id BIGINT NOT NULL,
    client_id TEXT NOT NULL,
    wallet_name TEXT NOT NULL,
    wallet_type TEXT NOT NULL,
    connected_at TIMESTAMPTZ NOT NULL,
);