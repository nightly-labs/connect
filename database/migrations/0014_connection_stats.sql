----------------- Hourly connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW hourly_connection_events WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 hour', connected_at) AS hourly_bucket,
    COUNT(*) AS hourly_connection_count
FROM
    connection_events
GROUP BY
    app_id,
    network,
    hourly_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('hourly_connection_events',
        start_offset => INTERVAL '2 day',
        end_offset => INTERVAL '1 hour',
        schedule_interval => INTERVAL '1 hour'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW hourly_requests_stats_per_app
set
    (timescaledb.materialized_only = false);



----------------- Daily connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW daily_connection_events WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 day', hourly_bucket) AS daily_bucket,
    SUM(hourly_connection_count) AS daily_connection_count
FROM
    hourly_connection_events
GROUP BY
    app_id,
    network,
    daily_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('daily_connection_events',
        start_offset => INTERVAL '1 month',
        end_offset => INTERVAL '1 day',
        schedule_interval => INTERVAL '1 day'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_connection_events
set
    (timescaledb.materialized_only = false);



----------------- Monthly connection per app -----------------
--- View
CREATE MATERIALIZED VIEW monthly_connection_events WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 month', daily_bucket) AS monthly_bucket,
    SUM(daily_connection_count) AS monthly_connection_count
FROM
    daily_connection_events
GROUP BY
    app_id,
    network,
    monthly_bucket WITH NO DATA;

--- Refresh policy
SELECT 
    add_continuous_aggregate_policy('monthly_connection_events',
        start_offset => INTERVAL '1 year',
        end_offset => INTERVAL '1 month',
        schedule_interval => INTERVAL '1 month'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW monthly_connection_events
set
    (timescaledb.materialized_only = false);