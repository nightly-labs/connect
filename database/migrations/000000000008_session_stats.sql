----------------- Sessions Stats -----------------
--- View
CREATE MATERIALIZED VIEW sessions_stats_per_app_daily
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, session_open_timestamp) AS daily_bucket,
    COUNT(*) AS num_opened_sessions,
    stats_agg(EXTRACT(EPOCH FROM (session_close_timestamp - session_open_timestamp))) AS avg_daily_session_duration_seconds
FROM
    sessions
WHERE
    session_close_timestamp IS NOT NULL
GROUP BY
    app_id, daily_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy('sessions_stats_per_app_daily',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

--- Real time aggregation
ALTER MATERIALIZED VIEW sessions_stats_per_app_daily SET (timescaledb.materialized_only = false);



----------------- Monthly Average session duration -----------------
--- View
CREATE MATERIALIZED VIEW sessions_stats_per_app_monthly
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month'::interval, daily_bucket) AS monthly_bucket,
    SUM(num_opened_sessions)::BIGINT AS total_opened_sessions_monthly,
    average(rollup(avg_daily_session_duration_seconds)) AS avg_monthly_session_duration_seconds,
    SUM(num_opened_sessions)::BIGINT / COUNT(DISTINCT daily_bucket) AS avg_daily_opened_sessions
FROM
    sessions_stats_per_app_daily
GROUP BY
    app_id, monthly_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy('sessions_stats_per_app_monthly',
    start_offset => INTERVAL '3 months',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

--- Real time aggregation
ALTER MATERIALIZED VIEW sessions_stats_per_app_monthly SET (timescaledb.materialized_only = false);
