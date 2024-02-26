CREATE TABLE requests(
    request_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    app_id TEXT NOT NULL,
    request_type TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    creation_timestamp TIMESTAMPTZ NOT NULL
);
