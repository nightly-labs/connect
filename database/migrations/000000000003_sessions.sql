CREATE TABLE sessions(
    session_id TEXT NOT NULL UNIQUE,
    app_id TEXT NOT NULL,
    app_metadata TEXT NOT NULL,
    app_connection_address TEXT NOT NULL,
    persistent BOOLEAN NOT NULL,
    network TEXT NOT NULL,
    client_id TEXT,
    client_device TEXT,
    client_metadata TEXT,
    client_notification_endpoint TEXT,
    client_connected_at BIGINT,
    session_open_timestamp BIGINT NOT NULL,
    session_close_timestamp BIGINT
);

CREATE UNIQUE INDEX sessions_session_id ON sessions(session_id);