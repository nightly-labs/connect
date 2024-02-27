CREATE TABLE registered_apps(
    team_id TEXT NOT NULL REFERENCES team(team_id) ON DELETE CASCADE,
    app_id TEXT NOT NULL UNIQUE,
    app_name TEXT NOT NULL,
    whitelisted_domains TEXT [] NOT NULL,
    ack_public_keys TEXT [] NOT NULL,
    email TEXT,
    registration_timestamp BIGINT NOT NULL,
    pass_hash TEXT
);

CREATE UNIQUE INDEX app_id_idx ON registered_apps(app_id);