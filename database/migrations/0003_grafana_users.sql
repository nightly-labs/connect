CREATE TABLE grafana_users(
    name TEXT NOT NULL UNIQUE,
    team_id TEXT NOT NULL REFERENCES team(team_id) ON DELETE CASCADE,
    team_admin BOOLEAN NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    privilege_level privilege_level_enum NOT NULL,
    creation_timestamp TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX grafana_users_name_idx ON grafana_users(name);