CREATE TABLE user_app_privileges (
    user_id TEXT NOT NULL,
    app_id TEXT NOT NULL,
    creation_timestamp TIMESTAMPTZ NOT NULL,
    privilege_level privilege_level_enum NOT NULL,
    PRIMARY KEY (user_id, app_id),
    FOREIGN KEY (user_id) REFERENCES grafana_users (user_id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES registered_apps (app_id) ON DELETE CASCADE
);
