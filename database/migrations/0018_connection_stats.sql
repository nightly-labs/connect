----------------- Quarter connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_connection_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('15 minutes' :: interval, connected_at) AS quarter_bucket,
    COUNT(*) FILTER (WHERE entity_type = 'App') :: BIGINT AS quarter_app_connection_count,
    COUNT(*) FILTER (WHERE entity_type = 'Client') :: BIGINT AS quarter_clients_connection_count
FROM
    connection_events
GROUP BY
    app_id,
    quarter_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('quarter_connection_stats_per_app',
        start_offset => INTERVAL '1 day',
        end_offset => INTERVAL '15 m',
        schedule_interval => INTERVAL '30 m'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_connection_stats_per_app
set
    (timescaledb.materialized_only = false);



----------------- Hourly connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW hourly_connection_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'  :: interval, quarter_bucket) AS hourly_bucket,
    SUM(quarter_app_connection_count) :: BIGINT AS hourly_app_connection_count,
    SUM(quarter_clients_connection_count) :: BIGINT AS hourly_clients_connection_count
FROM
    quarter_connection_stats_per_app
GROUP BY
    app_id,
    hourly_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('hourly_connection_stats_per_app',
        start_offset => INTERVAL '2 day',
        end_offset => INTERVAL '1 hour',
        schedule_interval => INTERVAL '1 hour'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW hourly_connection_stats_per_app
set
    (timescaledb.materialized_only = false);



----------------- Daily connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW daily_connection_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day' :: interval, hourly_bucket) AS daily_bucket,
    SUM(hourly_app_connection_count) :: BIGINT AS daily_app_connection_count,
    SUM(hourly_clients_connection_count) :: BIGINT AS daily_clients_connection_count
FROM
    hourly_connection_stats_per_app
GROUP BY
    app_id,
    daily_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('daily_connection_stats_per_app',
        start_offset => INTERVAL '1 month',
        end_offset => INTERVAL '1 day',
        schedule_interval => INTERVAL '1 day'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_connection_stats_per_app
set
    (timescaledb.materialized_only = false);



----------------- Monthly connection per app -----------------
--- View
CREATE MATERIALIZED VIEW monthly_connection_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
    SUM(daily_app_connection_count) :: BIGINT AS monthly_app_connection_count,
    SUM(daily_clients_connection_count) :: BIGINT AS monthly_clients_connection_count
FROM
    daily_connection_stats_per_app
GROUP BY
    app_id,
    monthly_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('monthly_connection_stats_per_app',
        start_offset => INTERVAL '1 year',
        end_offset => INTERVAL '1 month',
        schedule_interval => INTERVAL '1 month'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW monthly_connection_stats_per_app
set
    (timescaledb.materialized_only = false);



---------------------------------------------------------------- CONNECTION STATS BASED ON EVENT ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_client_connect_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    ecw.wallet_name,
    COUNT(*) FILTER (WHERE ecw.success = TRUE) AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE ecw.success = FALSE) AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_client_connect ecw ON e.event_id = ecw.event_id
GROUP BY e.app_id, quarter_bucket, ecw.wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_client_connect_wallet_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_client_connect_wallet_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_client_connect_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    wallet_name,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_client_connect_wallet_stats_per_app
GROUP BY app_id, hour_bucket, wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_client_connect_wallet_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_client_connect_wallet_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_client_connect_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    wallet_name,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_client_connect_wallet_stats_per_app
GROUP BY app_id, daily_bucket, wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_client_connect_wallet_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_client_connect_wallet_stats_per_app
SET (timescaledb.materialized_only = false);