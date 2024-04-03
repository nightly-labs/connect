CREATE TABLE domain_verifications(
    domain_name TEXT PRIMARY KEY,
    app_id TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ
);

CREATE INDEX domain_verifications_app_id_idx ON domain_verifications(app_id);
