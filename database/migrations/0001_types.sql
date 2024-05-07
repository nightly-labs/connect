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
    client_profile_id BIGINT,
    client_id TEXT,
    wallet_name TEXT,
    wallet_type TEXT,
    connected_at TIMESTAMPTZ
);

CREATE TYPE geo_location AS (
    country TEXT,
    city TEXT,
    lat FLOAT8,
    lon FLOAT8
);

CREATE TYPE event_type_enum AS ENUM (
    'AppConnect',
    'AppDisconnect',
    'ClientConnectInit',
    'ClientConnectResolve'
    'ClientDisconnect',
    'SignMessage',
    'SignTransaction',
    'SignAndSendTransaction',
    'ChangeWallet',
    'ChangeNetwork'
);

CREATE TYPE device_medium_type_enum AS ENUM ('Browser', 'Mobile', 'Unknown');