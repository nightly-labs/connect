CREATE TABLE subscriptions(
    email TEXT NOT NULL UNIQUE,
    subscribed_at BIGINT NOT NULL
);

CREATE UNIQUE INDEX subscriptions_email_idx ON subscriptions(email);