CREATE TABLE team(
    team_id TEXT NOT NULL UNIQUE,
    team_name TEXT NOT NULL,
    personal BOOLEAN NOT NULL,
    subscription subscription,
    team_admin_id TEXT NOT NULL,
    registration_timestamp TIMESTAMPTZ NOT NULL,
    deactivated_at TIMESTAMPTZ,
    PRIMARY KEY (team_name, team_admin_id)
);