CREATE TYPE request_status_enum AS ENUM (
    'Pending', 
    'Completed', 
    'Rejected', 
    'TimedOut'
);