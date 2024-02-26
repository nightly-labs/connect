----------------- Hourly requests per app -----------------
CREATE MATERIALIZED VIEW hourly_requests_per_app
WITH (timescaledb.continuous)
AS SELECT
    app_id,
    time_bucket('1 h'::interval, creation_timestamp) as hourly_bucket,
    COUNT(*) AS hourly_request_count
FROM requests
GROUP BY app_id, hourly_bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('hourly_requests_per_app',
    start_offset => INTERVAL '3 h',
    end_offset => INTERVAL '1 h',
    schedule_interval => INTERVAL '1 h');

----------------- Daily requests per app -----------------
CREATE MATERIALIZED VIEW daily_requests_per_app
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 d'::interval, hourly_bucket) AS daily_bucket,
    SUM(hourly_request_count)::BIGINT AS daily_request_count
FROM hourly_requests_per_app
GROUP BY app_id, daily_bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('daily_requests_per_app',
    start_offset => INTERVAL '3 d',
    end_offset => INTERVAL '1 h',
    schedule_interval => INTERVAL '12 h');

----------------- Monthly requests per app -----------------
CREATE MATERIALIZED VIEW monthly_requests_per_app
WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 month'::interval, daily_bucket) AS monthly_bucket,
    SUM(daily_request_count)::BIGINT AS monthly_request_count
FROM daily_requests_per_app
GROUP BY app_id, monthly_bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('monthly_requests_per_app',
    start_offset => INTERVAL '3 month',
    end_offset => INTERVAL '1 h',
    schedule_interval => INTERVAL '1 month');


