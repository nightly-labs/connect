---------------------------------------------------------------- CHANGE WALLET EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_change_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    e.network,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    ecw.wallet_name,
    COUNT(*) FILTER (WHERE ecw.request_status = 'Completed') AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE ecw.request_status != 'Completed') AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_change_wallet ecw ON e.event_id = ecw.event_id
GROUP BY e.app_id, e.network, quarter_bucket, ecw.wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_change_wallet_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_change_wallet_stats_per_app
SET (timescaledb.materialized_only = false);


------------------- Hourly events stats per app -----------------
CREATE MATERIALIZED VIEW hour_events_change_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    wallet_name,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_change_wallet_stats_per_app
GROUP BY app_id, network, hour_bucket, wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_change_wallet_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_change_wallet_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_change_wallet_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    wallet_name,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_change_wallet_stats_per_app
GROUP BY app_id, network, daily_bucket, wallet_name
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_change_wallet_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_change_wallet_stats_per_app
SET (timescaledb.materialized_only = false);



---------------------------------------------------------------- CHANGE NETWORK EVENT STATS ----------------------------------------------------------------
------------------- 15-minutes events stats per app -----------------
--- View
CREATE MATERIALIZED VIEW quarter_events_change_network_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    e.app_id,
    e.network,
    time_bucket('15 minutes'::interval, e.creation_timestamp) AS quarter_bucket,
    ecn.old_network,
    COUNT(*) FILTER (WHERE ecn.request_status = 'Completed') AS quarter_successful_requests,
    COUNT(*) FILTER (WHERE ecn.request_status != 'Completed') AS quarter_unsuccessful_requests
FROM
    events e
JOIN
    event_change_network ecn ON e.event_id = ecn.event_id
GROUP BY e.app_id, e.network, quarter_bucket, ecn.old_network
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'quarter_events_change_network_stats_per_app',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '30 minutes'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW quarter_events_change_network_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Hourly events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW hour_events_change_network_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 hour'::interval, quarter_bucket) AS hour_bucket,
    old_network,
    SUM(quarter_successful_requests) AS hour_successful_requests,
    SUM(quarter_unsuccessful_requests) AS hour_unsuccessful_requests
FROM
    quarter_events_change_network_stats_per_app
GROUP BY app_id, network, hour_bucket, old_network
WITH NO DATA;

--- Refresh policy
SELECT add_continuous_aggregate_policy(
    'hour_events_change_network_stats_per_app',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

--- Real time aggregation
ALTER MATERIALIZED VIEW hour_events_change_network_stats_per_app
SET (timescaledb.materialized_only = false);



------------------- Daily events stats per app -----------------
-- View
CREATE MATERIALIZED VIEW daily_events_change_network_stats_per_app WITH (timescaledb.continuous) AS
SELECT
    app_id,
    network,
    time_bucket('1 day'::interval, hour_bucket) AS daily_bucket,
    old_network,
    SUM(hour_successful_requests) AS daily_successful_requests,
    SUM(hour_unsuccessful_requests) AS daily_unsuccessful_requests
FROM
    hour_events_change_network_stats_per_app
GROUP BY app_id, network, daily_bucket, old_network 
WITH NO DATA;

-- Refresh policy
SELECT add_continuous_aggregate_policy(
    'daily_events_change_network_stats_per_app',
    start_offset => INTERVAL '14 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day'
);

-- Real time aggregation
ALTER MATERIALIZED VIEW daily_events_change_network_stats_per_app
SET (timescaledb.materialized_only = false);