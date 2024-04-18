---------------------------------------------------------------- SIGN MESSAGE EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_sign_message_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    e.event_type,
    COUNT(*) FILTER (WHERE esm.request_status = 'Completed') AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE esm.request_status != 'Completed') AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_sign_message esm
ON e.event_id = esm.event_id
GROUP BY e.app_id, quarter_bucket, e.event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_sign_message_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_sign_message_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Hourly events stats per app -----------------
-- Hourly aggregates
CREATE MATERIALIZED VIEW hour_events_sign_message_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    event_type,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_sign_message_stats_per_app
GROUP BY app_id, hour_bucket, event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_sign_message_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_sign_message_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_sign_message_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    event_type,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_sign_message_stats_per_app
GROUP BY app_id, daily_bucket, event_type WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_sign_message_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_sign_message_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- SIGN TRANSACTION EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_sign_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    e.event_type,
    COUNT(*) FILTER (WHERE esm.request_status = 'Completed') AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE esm.request_status != 'Completed') AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_sign_transaction esm
ON e.event_id = esm.event_id
GROUP BY e.app_id, quarter_bucket, e.event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_sign_transaction_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_sign_transaction_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Hourly events stats per app -----------------
-- Hourly aggregates
CREATE MATERIALIZED VIEW hour_events_sign_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    event_type,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_sign_transaction_stats_per_app
GROUP BY app_id, hour_bucket, event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_sign_transaction_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_sign_transaction_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_sign_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    event_type,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_sign_transaction_stats_per_app
GROUP BY app_id, daily_bucket, event_type WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_sign_transaction_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_sign_transaction_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- SIGN AND SEND TRANSACTION EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_sign_and_send_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    e.event_type,
    COUNT(*) FILTER (WHERE esm.request_status = 'Completed') AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE esm.request_status != 'Completed') AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_sign_and_send_transaction esm
ON e.event_id = esm.event_id
GROUP BY e.app_id, quarter_bucket, e.event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_sign_and_send_transaction_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_sign_and_send_transaction_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Hourly events stats per app -----------------
-- Hourly aggregates
CREATE MATERIALIZED VIEW hour_events_sign_and_send_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    event_type,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_sign_and_send_transaction_stats_per_app
GROUP BY app_id, hour_bucket, event_type WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_sign_and_send_transaction_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_sign_and_send_transaction_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_sign_and_send_transaction_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    event_type,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_sign_and_send_transaction_stats_per_app
GROUP BY app_id, daily_bucket, event_type WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_sign_and_send_transaction_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_sign_and_send_transaction_stats_per_app
SET (timescaledb.materialized_only = false);