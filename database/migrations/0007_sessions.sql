CREATE TABLE sessions (
    session_id TEXT NOT NULL,
    session_type session_type_enum NOT NULL,
    app_id TEXT NOT NULL,
    app_metadata TEXT NOT NULL,
    persistent BOOLEAN NOT NULL,
    network TEXT NOT NULL,
    client_data client_data,
    session_open_timestamp TIMESTAMPTZ NOT NULL,
    session_close_timestamp TIMESTAMPTZ
);

CREATE INDEX sessions_app_id_idx ON sessions(app_id);