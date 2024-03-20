CREATE TABLE event_app_connect(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    device_metadata TEXT NOT NULL,
    lang TEXT NOT NULL,
    timezone TEXT NOT NULL,
    new_session BOOLEAN NOT NULL,
);

CREATE TABLE event_app_disconnect(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
);

CREATE TABLE event_client_connect_init(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    session_type session_type_enum NOT NULL
);

CREATE TABLE event_client_connect_resolve(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    client_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    public_keys TEXT[] NOT NULL,
    wallet_name TEXT NOT NULL,
    wallet_type TEXT NOT NULL,
    session_type session_type_enum NOT NULL,
    success BOOLEAN NOT NULL
);

CREATE TABLE event_client_disconnect(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    disconnected_session_id TEXT NOT NULL
);

CREATE TABLE event_sign_message(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL
);

CREATE TABLE event_sign_transaction(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    transaction_id TEXT
);

CREATE TABLE event_sign_and_send_transaction(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    transaction_id TEXT
);

CREATE TABLE event_change_wallet(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    wallet_name TEXT NOT NULL,
    wallet_type TEXT NOT NULL
);

CREATE TABLE event_change_network(
    event_id BIGINT PRIMARY KEY REFERENCES events(event_id),
    session_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    new_network TEXT NOT NULL
);