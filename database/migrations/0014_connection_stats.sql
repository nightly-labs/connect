----------------- Hourly connection stats per app -----------------
--- View
CREATE MATERIALIZED VIEW hourly_connection_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour', connected_at) AS hourly_bucket,
    COUNT(*) FILTER (WHERE entity_type = 'App') :: BIGINT AS hourly_app_connection_count,
    COUNT(*) FILTER (WHERE entity_type = 'Client') :: BIGINT AS hourly_clients_connection_count
FROM
    connection_events
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
    time_bucket('1 day', hourly_bucket) AS daily_bucket,
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
    time_bucket('1 month', daily_bucket) AS monthly_bucket,
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