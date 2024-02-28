----------------- Hourly Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW hourly_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour' :: interval, session_open_timestamp) AS hourly_bucket,
    COUNT(*) :: BIGINT AS hourly_opened_sessions,
    COUNT(DISTINCT client_profile_id) FILTER (WHERE client_profile_id IS NOT NULL) :: BIGINT AS hourly_active_users
FROM
    sessions
GROUP BY
    app_id,
    hourly_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'hourly_sessions_stats_per_app',
        start_offset => INTERVAL '14 days',
        end_offset => INTERVAL '1 hour',
        schedule_interval => INTERVAL '1 hour'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW hourly_sessions_stats_per_app
SET
    (timescaledb.materialized_only = false);



----------------- Daily Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW daily_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day' :: interval, hourly_bucket) AS daily_bucket,
    SUM(hourly_opened_sessions) :: BIGINT AS daily_opened_sessions,
    SUM(hourly_active_users) :: BIGINT AS daily_active_users
FROM
    hourly_sessions_stats_per_app
GROUP BY
    app_id,
    daily_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'daily_sessions_stats_per_app',
        start_offset => INTERVAL '14 days',
        end_offset => INTERVAL '1 hour',
        schedule_interval => INTERVAL '1 hour'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_sessions_stats_per_app
SET
    (timescaledb.materialized_only = false);

----------------- Monthly session stats -----------------
--- View
CREATE MATERIALIZED VIEW monthly_sessions_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
    SUM(daily_opened_sessions) :: BIGINT AS monthly_opened_sessions,
    SUM(daily_active_users) :: BIGINT AS monthly_active_users
FROM
    daily_sessions_stats_per_app
GROUP BY
    app_id,
    monthly_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'monthly_sessions_stats_per_app',
        start_offset => INTERVAL '3 months',
        end_offset => INTERVAL '1 day',
        schedule_interval => INTERVAL '1 day'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW monthly_sessions_stats_per_app
SET
    (timescaledb.materialized_only = false);




----------------- Daily Average session duration -----------------
--- View
CREATE MATERIALIZED VIEW daily_sessions_avg_time_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day' :: interval, session_open_timestamp) AS daily_bucket,
    stats_agg(
        EXTRACT(
            EPOCH
            FROM
                (session_close_timestamp - session_open_timestamp)
        )
    ) AS daily_avg_session_duration_seconds
FROM
    sessions
WHERE
    session_close_timestamp IS NOT NULL
GROUP BY
    app_id,
    daily_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'daily_sessions_avg_time_per_app',
        start_offset => INTERVAL '14 days',
        end_offset => INTERVAL '1 hour',
        schedule_interval => INTERVAL '1 hour'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_sessions_avg_time_per_app
SET
    (timescaledb.materialized_only = false);



----------------- Monthly Average session duration -----------------
--- View
CREATE MATERIALIZED VIEW monthly_sessions_avg_time_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
    average(rollup(daily_avg_session_duration_seconds)) AS monthly_avg_session_duration_seconds
FROM
    daily_sessions_avg_time_per_app
GROUP BY
    app_id,
    monthly_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'monthly_sessions_avg_time_per_app',
        start_offset => INTERVAL '3 months',
        end_offset => INTERVAL '1 day',
        schedule_interval => INTERVAL '1 day'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW monthly_sessions_avg_time_per_app
SET
    (timescaledb.materialized_only = false);
