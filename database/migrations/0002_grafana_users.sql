CREATE TABLE grafana_user(
    name TEXT NOT NULL,
    team_id TEXT NOT NULL REFERENCES team(team_id) ON DELETE CASCADE,
    team_admin BOOLEAN NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    privilege_level privilege_level_enum NOT NULL
);