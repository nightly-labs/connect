----------------- Hourly requests stats per app -----------------
--- View
CREATE MATERIALIZED VIEW hourly_requests_stats WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour' :: interval, creation_timestamp) AS hourly_bucket,
    COUNT(*) AS hourly_request_count,
    COUNT(*) FILTER (
        WHERE
            request_status = 'Completed'
    ) :: FLOAT / NULLIF(
        COUNT(*) FILTER (
            WHERE
                request_status IN ('Completed', 'Rejected', 'TimedOut')
        ),
        0
    ) AS success_rate
FROM
    requests
GROUP BY
    app_id,
    hourly_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'hourly_requests_stats',
        start_offset => INTERVAL '3 h',
        end_offset => INTERVAL '1 h',
        schedule_interval => INTERVAL '1 h'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW hourly_requests_stats
set
    (timescaledb.materialized_only = false);

----------------- Daily requests stats per app -----------------
--- View
CREATE MATERIALIZED VIEW daily_requests_stats WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day' :: interval, hourly_bucket) AS daily_bucket,
    SUM(hourly_request_count) :: BIGINT AS daily_request_count,
    SUM(hourly_request_count * success_rate) :: FLOAT / SUM(hourly_request_count) AS success_rate
FROM
    hourly_requests_stats
GROUP BY
    app_id,
    daily_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'daily_requests_stats',
        start_offset => INTERVAL '3 d',
        end_offset => INTERVAL '1 h',
        schedule_interval => INTERVAL '12 h'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_requests_stats
set
    (timescaledb.materialized_only = false);

----------------- Monthly requests per app -----------------
--- View
CREATE MATERIALIZED VIEW monthly_requests_stats WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month' :: interval, daily_bucket) AS monthly_bucket,
    SUM(daily_request_count) :: BIGINT AS monthly_request_count,
    SUM(daily_request_count * success_rate) :: FLOAT / SUM(daily_request_count) AS success_rate
FROM
    daily_requests_stats
GROUP BY
    app_id,
    monthly_bucket WITH NO DATA;

--- Refresh policy
SELECT
    add_continuous_aggregate_policy(
        'monthly_requests_stats',
        start_offset => INTERVAL '3 month',
        end_offset => INTERVAL '1 h',
        schedule_interval => INTERVAL '1 month'
    );

--- Real time aggregation
ALTER MATERIALIZED VIEW monthly_requests_stats
set
    (timescaledb.materialized_only = false);