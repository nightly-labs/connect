CREATE TYPE request_status_enum AS ENUM (
    'Pending', 
    'Completed', 
    'Failed', 
    'Rejected', 
    'TimedOut', 
    'Unknown'
);