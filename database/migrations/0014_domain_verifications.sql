CREATE TABLE domain_verifications(
    domain_name TEXT NOT NULL,
    app_id TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ,
    PRIMARY KEY (domain_name, app_id) -- One app can only verify particular domain once 
);

-- Safety measure to prevent verification blockade in case of malicious intent
CREATE UNIQUE INDEX idx_unique_verified_domains ON domain_verifications (domain_name)
WHERE finished_at IS NOT NULL;

CREATE INDEX domain_verifications_app_id_idx ON domain_verifications(app_id);
