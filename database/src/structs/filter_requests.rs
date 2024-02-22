use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct AggregatedRequestCount {
    pub app_id: String,
    pub bucket: DateTime<Utc>,
    pub request_count: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionDurationAverage {
    pub app_id: String,
    pub bucket: DateTime<Utc>,
    pub average_duration_seconds: f64,
}
