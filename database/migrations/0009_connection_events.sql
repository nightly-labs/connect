CREATE TABLE connection_events (
    event_id BIGSERIAL NOT NULL,
    app_id TEXT NOT NULL, -- Always references the related app, whether an app or client connection
    session_id TEXT NOT NULL,
    connection_id TEXT, -- Unique for each connection instance, NULL for clients, UNIQUE per app connection
    entity_id TEXT NOT NULL, -- The ID of the connecting entity (could be the same app_id or client_profile_id)
    entity_type entity_type_enum NOT NULL, -- Distinguishes between 'client' and 'app'
    network TEXT NOT NULL,
    connected_at TIMESTAMPTZ NOT NULL,
    disconnected_at TIMESTAMPTZ
);