CREATE TABLE event_app_connect(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    device_medium_type device_medium_type_enum,
    device_metadata_uuid TEXT,
    lang TEXT NOT NULL,
    timezone TEXT NOT NULL,
    new_session BOOLEAN NOT NULL
);

CREATE TABLE event_app_disconnect(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL
);

CREATE TABLE event_client_connect(
    event_id BIGINT PRIMARY KEY,
    client_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    addresses TEXT[],
    wallet_name TEXT NOT NULL,
    wallet_type TEXT NOT NULL,
    session_type session_type_enum NOT NULL,
    success BOOLEAN NOT NULL
);

CREATE TABLE event_client_disconnect(
    event_id BIGINT PRIMARY KEY,
    client_id TEXT NOT NULL,
    disconnected_session_id TEXT NOT NULL
);

CREATE TABLE event_sign_message(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL
);

CREATE TABLE event_sign_transaction(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    tx_hash TEXT
);

CREATE TABLE event_sign_and_send_transaction(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    tx_hash TEXT
);

CREATE TABLE event_change_wallet(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    wallet_name TEXT NOT NULL,
    wallet_type TEXT NOT NULL,
    old_wallet_address TEXT NOT NULL,
    new_wallet_address TEXT
);

CREATE TABLE event_change_network(
    event_id BIGINT PRIMARY KEY,
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    old_network TEXT NOT NULL,
    new_network TEXT
);