use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const REGISTERED_APPS_TABLE_NAME: &str = "registered_apps";
pub const REGISTERED_APPS_KEYS: &str =
    "team_id, app_id, app_name, whitelisted_domains, ack_public_keys, registration_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbRegisteredApp {
    pub team_id: String,
    pub app_id: String,
    pub app_name: String,
    pub whitelisted_domains: Vec<String>,
    pub ack_public_keys: Vec<String>,
    pub registration_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for DbRegisteredApp {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(DbRegisteredApp {
            team_id: row.get("team_id"),
            app_id: row.get("app_id"),
            app_name: row.get("app_name"),
            whitelisted_domains: row.get("whitelisted_domains"),
            ack_public_keys: row.get("ack_public_keys"),
            registration_timestamp: row.get("registration_timestamp"),
        })
    }
}
