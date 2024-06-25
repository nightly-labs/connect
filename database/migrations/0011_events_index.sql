CREATE TABLE events(
    event_id BIGSERIAL,
    app_id TEXT NOT NULL,
    network TEXT NOT NULL,
    event_type event_type_enum NOT NULL,
    creation_timestamp TIMESTAMPTZ NOT NULL
);

CREATE TABLE web_metadata(
    uuid TEXT UNIQUE NOT NULL,
    browser TEXT NOT NULL,
    browser_version TEXT NOT NULL,
    os TEXT NOT NULL,
    os_version TEXT NOT NULL
);

CREATE TABLE mobile_metadata(
    uuid TEXT UNIQUE NOT NULL,
    system_type TEXT NOT NULL,
    system_version TEXT NOT NULL
);