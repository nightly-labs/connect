CREATE TABLE events(
    event_id BIGSERIAL PRIMARY KEY,
    app_id TEXT NOT NULL,
    event_type event_type_enum NOT NULL,
    creation_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);