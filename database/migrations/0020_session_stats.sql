----------------- 15-minutes Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW quarter_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('15 minutes' :: interval, session_open_timestamp) AS quarter_bucket,
    COUNT(*) :: BIGINT AS quarter_sessions_opened,
    approx_count_distinct((client_data).client_profile_id) AS quarter_active_users
FROM
    sessions
GROUP BY
    app_id,
    network,
    quarter_bucket WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_sessions_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 m',
    schedule_interval => INTERVAL '30 m'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW quarter_sessions_stats_per_app
SET (timescaledb.materialized_only = false);



----------------- Hourly Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW hourly_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 hour' :: interval, quarter_bucket) AS hourly_bucket,
    SUM(quarter_sessions_opened) AS hourly_sessions_opened,
    rollup(quarter_active_users) AS hourly_active_users
FROM
    quarter_sessions_stats_per_app
GROUP BY
    app_id,
    network,
    hourly_bucket WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hourly_sessions_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW hourly_sessions_stats_per_app
SET (timescaledb.materialized_only = false);



----------------- Daily Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW daily_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 day' :: interval, hourly_bucket) AS daily_bucket,
    SUM(hourly_sessions_opened) AS daily_sessions_opened,
    -- SUM(hourly_distinct_users) AS daily_active_users
    distinct_count(rollup(hourly_active_users)) AS daily_active_users
FROM
    hourly_sessions_stats_per_app
GROUP BY
    app_id,
    network,
    daily_bucket WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_sessions_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW daily_sessions_stats_per_app
SET (timescaledb.materialized_only = false);



-- ----------------- Monthly session stats -----------------
-- --- View
-- CREATE MATERIALIZED VIEW monthly_sessions_stats_per_app WITH (timescaledb.continuous) AS
-- SELECT
--     app_id,
--     time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
--     SUM(daily_sessions_opened) :: BIGINT AS monthly_sessions_opened,
--     SUM(daily_active_users) :: BIGINT AS monthly_active_users
-- FROM
--     daily_sessions_stats_per_app
-- GROUP BY
--     app_id,
--     monthly_bucket WITH NO DATA;

-- --- Refresh policy
-- SELECT
--     add_continuous_aggregate_policy(
--         'monthly_sessions_stats_per_app',
--         start_offset => INTERVAL '3 months',
--         end_offset => INTERVAL '1 day',
--         schedule_interval => INTERVAL '1 day'
--     );

-- --- Real time aggregation
-- ALTER MATERIALIZED VIEW monthly_sessions_stats_per_app
-- SET
--     (timescaledb.materialized_only = false);



-- ------------------------------------------------------------------
-- ----------------- Daily Average session duration -----------------
-- --- View
-- CREATE MATERIALIZED VIEW daily_sessions_avg_time_per_app WITH (timescaledb.continuous) AS
-- SELECT
--     app_id,
--     time_bucket('1 day' :: interval, session_open_timestamp) AS daily_bucket,
--     stats_agg(
--         EXTRACT(
--             EPOCH
--             FROM
--                 (session_close_timestamp - session_open_timestamp)
--         )
--     ) AS daily_avg_session_duration_seconds
-- FROM
--     sessions
-- WHERE
--     session_close_timestamp IS NOT NULL
-- GROUP BY
--     app_id,
--     daily_bucket WITH NO DATA;

-- --- Refresh policy
-- SELECT
--     add_continuous_aggregate_policy(
--         'daily_sessions_avg_time_per_app',
--         start_offset => INTERVAL '14 days',
--         end_offset => INTERVAL '1 hour',
--         schedule_interval => INTERVAL '1 hour'
--     );

-- --- Real time aggregation
-- ALTER MATERIALIZED VIEW daily_sessions_avg_time_per_app
-- SET
--     (timescaledb.materialized_only = false);



-- ----------------- Monthly Average session duration -----------------
-- --- View
-- CREATE MATERIALIZED VIEW monthly_sessions_avg_time_per_app WITH (timescaledb.continuous) AS
-- SELECT
--     app_id,
--     time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
--     average(rollup(daily_avg_session_duration_seconds)) AS monthly_avg_session_duration_seconds
-- FROM
--     daily_sessions_avg_time_per_app
-- GROUP BY
--     app_id,
--     monthly_bucket WITH NO DATA;

-- --- Refresh policy
-- SELECT
--     add_continuous_aggregate_policy(
--         'monthly_sessions_avg_time_per_app',
--         start_offset => INTERVAL '3 months',
--         end_offset => INTERVAL '1 day',
--         schedule_interval => INTERVAL '1 day'
--     );

-- --- Real time aggregation
-- ALTER MATERIALIZED VIEW monthly_sessions_avg_time_per_app
-- SET
--     (timescaledb.materialized_only = false);