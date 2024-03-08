CREATE TABLE grafana_users(
    user_id TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    creation_timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX grafana_users_name_idx ON grafana_users(user_id);
CREATE INDEX grafana_users_email_idx ON grafana_users(email);