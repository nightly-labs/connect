CREATE TABLE team_invites(
    invite_id SERIAL PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES team(team_id),
    user_email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

CREATE INDEX team_invites_user_email_idx ON team_invites(user_email);
CREATE INDEX team_invites_team_id_idx ON team_invites(team_id);
-- This will ensure that a user can only be invited to a team once
CREATE UNIQUE INDEX team_invites_team_id_user_email_idx ON team_invites(team_id, user_email);
