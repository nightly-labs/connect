CREATE TABLE domain_verifications(
    domain_name TEXT NOT NULL,
    app_id TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

-- Safety measure to prevent verification blockade in case of malicious intent
CREATE UNIQUE INDEX idx_unique_verified_domains ON domain_verifications (domain_name)
WHERE finished_at IS NOT NULL AND deleted_at IS NULL;

CREATE INDEX domain_verifications_app_id_idx ON domain_verifications(app_id);

-- Safety measure to prevent multiple verification requests for the same domain for the same app
CREATE UNIQUE INDEX idx_unique_active_domain_verifications 
ON domain_verifications (domain_name, app_id) 
WHERE finished_at IS NULL AND cancelled_at IS NULL AND deleted_at IS NULL;