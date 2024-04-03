use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const DOMAIN_VERIFICATIONS_TABLE_NAME: &str = "domain_verifications";
pub const DOMAIN_VERIFICATIONS_KEYS: &str = "domain_name, app_id, code, created_at, finished_at";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainVerification {
    pub domain_name: String,
    pub app_id: i64,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for DomainVerification {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(DomainVerification {
            domain_name: row.get("domain_name"),
            app_id: row.get("app_id"),
            code: row.get("code"),
            created_at: row.get("created_at"),
            finished_at: row.get("finished_at"),
        })
    }
}
