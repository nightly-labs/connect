CREATE TABLE users(
    user_id TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    passkeys TEXT,
    creation_timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX users_name_idx ON users(user_id);
CREATE INDEX users_email_idx ON users(email);