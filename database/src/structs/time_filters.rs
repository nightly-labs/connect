use super::consts::DAY_IN_SECONDS;
use sqlx::types::chrono::{NaiveDate, Utc};
use std::time::Duration;

pub enum TimeFilter {
    Last24Hours,
    Last7Days,
    Last30Days,
    LastMonth,
}

impl TimeFilter {
    pub fn to_date(&self) -> NaiveDate {
        let duration = match self {
            TimeFilter::Last24Hours => Duration::from_secs(DAY_IN_SECONDS),
            TimeFilter::Last7Days => Duration::from_secs(7 * DAY_IN_SECONDS),
            TimeFilter::Last30Days => Duration::from_secs(30 * DAY_IN_SECONDS),
            TimeFilter::LastMonth => Duration::from_secs(32 * DAY_IN_SECONDS),
        };
        // Subtract the duration from the current time and convert to NaiveDate
        (Utc::now() - duration).date_naive()
    }

    pub fn bucket_size(&self) -> &'static str {
        match self {
            TimeFilter::Last24Hours => "1 hour",
            TimeFilter::Last7Days => "1 day",
            TimeFilter::Last30Days => "1 day",
            TimeFilter::LastMonth => "1 month",
        }
    }
}
