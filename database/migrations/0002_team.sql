CREATE TABLE team(
    team_id TEXT NOT NULL UNIQUE,
    personal BOOLEAN NOT NULL,
    subscription subscription,
    team_admin_id TEXT NOT NULL,
    registration_timestamp TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX team_id_idx ON team(team_id);