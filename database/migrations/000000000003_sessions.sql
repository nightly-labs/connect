CREATE TABLE sessions(
    session_id TEXT NOT NULL,
    app_id TEXT NOT NULL REFERENCES registered_apps(app_id),
    app_metadata TEXT NOT NULL,
    app_ip_address TEXT NOT NULL,
    persistent BOOLEAN NOT NULL,
    network TEXT NOT NULL,
    client_id TEXT,
    client_device TEXT,
    client_metadata TEXT,
    client_notification_endpoint TEXT,
    client_connected_at TIMESTAMPTZ,
    session_open_timestamp TIMESTAMPTZ NOT NULL,
    session_close_timestamp TIMESTAMPTZ
);