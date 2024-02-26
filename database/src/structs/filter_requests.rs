use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct AggregatedRequestCount {
    pub app_id: String,
    pub bucket: DateTime<Utc>,
    pub request_count: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RequestsStats {
    pub app_id: String,
    pub bucket: DateTime<Utc>,
    pub request_count: i64,
    pub success_rate: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionsStats {
    pub app_id: String,
    pub bucket: DateTime<Utc>,
    pub sessions_opened: i64,
    pub average_duration_seconds: f64,
    pub avg_daily_opened_sessions: i64,
}
