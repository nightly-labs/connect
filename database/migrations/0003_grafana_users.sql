CREATE TABLE grafana_users(
    user_id TEXT NOT NULL UNIQUE,
    email TEXT,
    password_hash TEXT,
    creation_timestamp TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX grafana_users_name_idx ON grafana_users(user_id);