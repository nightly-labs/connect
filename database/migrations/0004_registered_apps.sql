CREATE TABLE registered_apps(
    team_id TEXT NOT NULL REFERENCES team(team_id),
    app_id TEXT NOT NULL UNIQUE,
    app_name TEXT NOT NULL,
    whitelisted_domains TEXT [] NOT NULL,
    ack_public_keys TEXT [] NOT NULL,
    registration_timestamp TIMESTAMPTZ NOT NULL,
    active BOOLEAN NOT NULL,
    deactivated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX app_id_idx ON registered_apps(app_id);