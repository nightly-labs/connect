CREATE TABLE public_keys (
    public_key TEXT NOT NULL UNIQUE,
    client_profile_id BIGINT NOT NULL REFERENCES client_profiles(client_profile_id),
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE session_public_keys (
    session_public_key_id BIGSERIAL PRIMARY KEY,
    session_id TEXT NOT NULL,
    public_key TEXT NOT NULL,
    key_belongs_to_profile BIGINT,
    main_session_key BOOLEAN NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_session_public_keys_session_id ON session_public_keys (session_id);
CREATE INDEX idx_session_public_keys_public_key ON session_public_keys (public_key);
