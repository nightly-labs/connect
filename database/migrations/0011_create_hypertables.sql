SELECT
    create_hypertable('sessions', 'session_open_timestamp');

SELECT
    create_hypertable('requests', 'creation_timestamp');