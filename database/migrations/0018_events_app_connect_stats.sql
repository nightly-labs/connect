---------------------------------------------------------------- APP CONNECT(LANGUAGE) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_language_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    eac.lang,
    COUNT(*) AS quarter_language
FROM
    events e
JOIN
    event_app_connect eac ON e.event_id = eac.event_id
GROUP BY e.app_id, quarter_bucket, eac.lang
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_app_connect_language_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_app_connect_language_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_app_connect_language_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    lang,
    SUM(quarter_language) AS hour_language
FROM
    quarter_events_app_connect_language_stats_per_app
GROUP BY app_id, hour_bucket, lang
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_app_connect_language_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_app_connect_language_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_app_connect_language_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    lang,
    SUM(hour_language) AS daily_language
FROM
    hour_events_app_connect_language_stats_per_app
GROUP BY app_id, daily_bucket, lang
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_app_connect_language_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_app_connect_language_stats_per_app
SET (timescaledb.materialized_only = false);



