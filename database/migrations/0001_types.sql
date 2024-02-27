CREATE TYPE subscription AS (
    subscription_type TEXT,
    valid_from BIGINT,
    valid_till BIGINT
);

CREATE TYPE request_status_enum AS ENUM (
    'Pending',
    'Completed',
    'Rejected',
    'TimedOut'
);

CREATE TYPE privilege_level_enum AS ENUM ('Read', 'Edit', 'Admin');