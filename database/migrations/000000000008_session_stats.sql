----------------- Daily Average session duration -----------------
--- View
CREATE MATERIALIZED VIEW avg_session_duration_per_app_daily
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, session_open_timestamp) AS daily_bucket,
    stats_agg(EXTRACT(EPOCH FROM (session_close_timestamp - session_open_timestamp))) AS daily_session_stats
FROM
    sessions
WHERE
    session_close_timestamp IS NOT NULL
GROUP BY
    app_id, daily_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy('avg_session_duration_per_app_daily',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

--- Real time aggregation
ALTER MATERIALIZED VIEW avg_session_duration_per_app_daily set (timescaledb.materialized_only = false);

----------------- Monthly Average session duration -----------------
--- View
CREATE MATERIALIZED VIEW avg_session_duration_per_app_monthly
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month'::interval, daily_bucket) AS monthly_bucket,
    average(rollup(daily_session_stats)) AS avg_monthly_session_duration_seconds
FROM
    avg_session_duration_per_app_daily
GROUP BY
    app_id, monthly_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy('avg_session_duration_per_app_monthly',
    start_offset => INTERVAL '3 months',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

--- Real time aggregation
ALTER MATERIALIZED VIEW avg_session_duration_per_app_monthly set (timescaledb.materialized_only = false);
