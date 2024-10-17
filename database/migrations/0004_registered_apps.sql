CREATE TABLE registered_apps(
    team_id TEXT NOT NULL REFERENCES team(team_id),
    app_id TEXT NOT NULL UNIQUE,
    app_name TEXT NOT NULL,
    whitelisted_domains TEXT [] NOT NULL,
    ack_public_keys TEXT [] NOT NULL,
    registration_timestamp TIMESTAMPTZ NOT NULL,
    deactivated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX unique_active_app_name 
ON registered_apps (team_id, app_name) 
WHERE deactivated_at IS NULL;