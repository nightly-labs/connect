CREATE TABLE requests(
    request_id TEXT NOT NULL UNIQUE,
    request_type TEXT NOT NULL,
    session_id TEXT NOT NULL,
    request_status request_status_enum NOT NULL,
    network TEXT NOT NULL,
    creation_timestamp BIGINT NOT NULL
);

CREATE UNIQUE INDEX requests_request_id ON requests(request_id);

ALTER TABLE requests
ADD CONSTRAINT fk_requests_sessions
FOREIGN KEY (session_id)
REFERENCES sessions (session_id)
ON DELETE CASCADE;