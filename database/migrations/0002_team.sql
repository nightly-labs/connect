CREATE TABLE team(
    team_id TEXT NOT NULL UNIQUE,
    team_name TEXT NOT NULL,
    personal BOOLEAN NOT NULL,
    subscription subscription,
    team_admin_id TEXT NOT NULL,
    registration_timestamp TIMESTAMPTZ NOT NULL,
    deactivated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX unique_active_team_name 
ON team (team_admin_id, team_name) 
WHERE deactivated_at IS NULL;