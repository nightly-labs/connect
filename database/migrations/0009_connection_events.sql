CREATE TABLE connection_events (
    event_id BIGSERIAL PRIMARY KEY,
    session_id TEXT NOT NULL,
    connection_id TEXT, -- NULL for clients, unique for app connections
    entity_id TEXT NOT NULL, -- client_profile_id or app_id
    entity_type entity_type_enum NOT NULL, -- 'client' or 'app'
    network TEXT NOT NULL,
    connected_at TIMESTAMPTZ NOT NULL,
    disconnected_at TIMESTAMPTZ
);

CREATE INDEX idx_connection_events_session ON connection_events(session_id);
CREATE INDEX idx_connection_events_entity ON connection_events(entity_id, entity_type);