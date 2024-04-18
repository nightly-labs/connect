SELECT
    create_hypertable('connection_events', 'connected_at');

-- SELECT
--     create_hypertable('requests', 'creation_timestamp');

SELECT
    create_hypertable('sessions', 'session_open_timestamp');

SELECT
    create_hypertable('events', 'creation_timestamp');

-- SELECT
--     create_hypertable('event_sign_message', 'creation_timestamp');