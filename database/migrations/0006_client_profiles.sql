CREATE TABLE client_profiles (
    client_profile_id BIGSERIAL PRIMARY KEY,
    merged_into_client_profile_id BIGINT REFERENCES client_profiles(client_profile_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE client_profile_merge_events (
    merge_event_id BIGSERIAL PRIMARY KEY,
    source_client_profile_id BIGINT NOT NULL REFERENCES client_profiles (client_profile_id),
    current_target_client_profile_id BIGINT REFERENCES client_profiles (client_profile_id),
    new_target_client_profile_id BIGINT NOT NULL REFERENCES client_profiles (client_profile_id),
    merge_detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);