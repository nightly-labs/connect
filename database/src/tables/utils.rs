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

// This function is used to format the keys of a table to be used in a view query
pub fn format_view_keys(prefix: &str, keys: &[(&'static str, bool)]) -> String {
    keys.iter()
        .map(|(key, add_prefix)| {
            if *add_prefix {
                format!("{}_{} as {}", prefix, key, key)
            } else {
                format!("{}", key)
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn format_view_name(prefix: &str, view_name: &str) -> String {
    format!("{}_{}", prefix, view_name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_view_keys() {
        let test_keys = [
            ("app_id", false),
            ("bucket", true),
            ("sessions_opened", true),
            ("active_users", true),
        ];

        let prefix = "hourly";
        let expected = "app_id, hourly_bucket as bucket, hourly_sessions_opened as sessions_opened, hourly_active_users as active_users";
        assert_eq!(format_view_keys(prefix, &test_keys), expected);

        let test_keys = [
            ("app_id", false),
            ("bucket", true),
            ("sessions_opened", false),
            ("active_users", true),
        ];
        let prefix = "daily";
        let expected =
            "app_id, daily_bucket as bucket, sessions_opened, daily_active_users as active_users";
        assert_eq!(format_view_keys(prefix, &test_keys), expected);
    }

    #[test]
    fn test_format_view_name() {
        let prefix = "hourly";
        let view_name = "sessions";
        let expected = "hourly_sessions";
        assert_eq!(format_view_name(prefix, view_name), expected);

        let prefix = "daily";
        let view_name = "sessions";
        let expected = "daily_sessions";
        assert_eq!(format_view_name(prefix, view_name), expected);
    }
}
