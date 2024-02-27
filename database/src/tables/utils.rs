use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp_in_milliseconds() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as u64
}

pub fn get_date_time(timestamp: u64) -> Option<DateTime<Utc>> {
    Utc.timestamp_millis_opt(timestamp as i64).single()
}

pub fn to_microsecond_precision(datetime: &DateTime<Utc>) -> DateTime<Utc> {
    // Should never fail as we are converting from a valid DateTime<Utc>
    Utc.timestamp_micros(datetime.timestamp_micros()).unwrap()
}
