---------------------------------------------------------------- APP CONNECT(LANGUAGE) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_language_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('15 minutes'::interval, creation_timestamp) AS quarter_bucket,
    lang,
    COUNT(*) AS quarter_language
FROM
    event_app_connect
GROUP BY app_id, quarter_bucket, lang
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



---------------------------------------------------------------- APP CONNECT(WEB SESSION BROWSER) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_browser_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    eac.app_id,
    time_bucket('15 minutes'::interval, eac.creation_timestamp) AS quarter_bucket,
    wm.browser,
    COUNT(*) AS quarter_browser
FROM
    event_app_connect eac
JOIN
    web_metadata wm ON eac.device_metadata_uuid = wm.uuid
GROUP BY eac.app_id, quarter_bucket, wm.browser
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_app_connect_browser_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_app_connect_browser_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_app_connect_browser_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    browser,
    SUM(quarter_browser) AS hour_browser
FROM
    quarter_events_app_connect_browser_stats_per_app
GROUP BY app_id, hour_bucket, browser
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_app_connect_browser_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_app_connect_browser_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_app_connect_browser_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    browser,
    SUM(hour_browser) AS daily_browser
FROM
    hour_events_app_connect_browser_stats_per_app
GROUP BY app_id, daily_bucket, browser
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_app_connect_browser_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_app_connect_browser_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- APP CONNECT(WEB SESSION SYSTEM OS) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_os_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    eac.app_id,
    time_bucket('15 minutes'::interval, eac.creation_timestamp) AS quarter_bucket,
    wm.os,
    COUNT(*) AS quarter_os
FROM
    event_app_connect eac
JOIN
    web_metadata wm ON eac.device_metadata_uuid = wm.uuid
GROUP BY eac.app_id, quarter_bucket, wm.os
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_app_connect_os_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_app_connect_os_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_app_connect_os_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    os,
    SUM(quarter_os) AS hour_os
FROM
    quarter_events_app_connect_os_stats_per_app
GROUP BY app_id, hour_bucket, os
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_app_connect_os_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_app_connect_os_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_app_connect_os_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    os,
    SUM(hour_os) AS daily_os
FROM
    hour_events_app_connect_os_stats_per_app
GROUP BY app_id, daily_bucket, os
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_app_connect_os_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_app_connect_os_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- APP CONNECT(MOBILE SESSION SYSTEM) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_system_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    eac.app_id,
    time_bucket('15 minutes'::interval, eac.creation_timestamp) AS quarter_bucket,
    mm.system_type,
    COUNT(*) AS quarter_system
FROM
    event_app_connect eac
JOIN
    mobile_metadata mm ON eac.device_metadata_uuid = mm.uuid
GROUP BY eac.app_id, quarter_bucket, mm.system_type
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_app_connect_system_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_app_connect_system_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_app_connect_system_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    system_type,
    SUM(quarter_system) AS hour_system
FROM
    quarter_events_app_connect_system_stats_per_app
GROUP BY app_id, hour_bucket, system_type
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_app_connect_system_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_app_connect_system_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_app_connect_system_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    system_type,
    SUM(hour_system) AS daily_system
FROM
    hour_events_app_connect_system_stats_per_app
GROUP BY app_id, daily_bucket, system_type
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_app_connect_system_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_app_connect_system_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- APP CONNECT(SESSION TYPE(MOBILE vs WEB)) EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_app_connect_session_type_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('15 minutes'::interval, creation_timestamp) AS quarter_bucket,
    COUNT(*) FILTER (WHERE device_medium_type = 'Browser') AS quarter_mobile_sessions,
    COUNT(*) FILTER (WHERE device_medium_type = 'Mobile') AS quarter_web_sessions,
    COUNT(*) FILTER (WHERE device_medium_type = 'Unknown') AS quarter_unknown_sessions
FROM
    event_app_connect
GROUP BY app_id, quarter_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_app_connect_session_type_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_app_connect_session_type_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_app_connect_session_type_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    SUM(quarter_web_sessions) AS hour_web_sessions,
    SUM(quarter_mobile_sessions) AS hour_mobile_sessions,
    SUM(quarter_unknown_sessions) AS hour_unknown_sessions
FROM
    quarter_events_app_connect_session_type_stats_per_app
GROUP BY app_id, hour_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_app_connect_session_type_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_app_connect_session_type_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_app_connect_session_type_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    SUM(hour_web_sessions) AS daily_web_sessions,
    SUM(hour_mobile_sessions) AS daily_mobile_sessions,
    SUM(hour_unknown_sessions) AS daily_unknown_sessions
FROM
    hour_events_app_connect_session_type_stats_per_app
GROUP BY app_id, daily_bucket
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_app_connect_session_type_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_app_connect_session_type_stats_per_app
SET (timescaledb.materialized_only = false);


