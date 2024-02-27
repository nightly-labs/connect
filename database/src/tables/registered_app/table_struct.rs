use crate::structs::subscription::Subscription;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const REGISTERED_APPS_TABLE_NAME: &str = "registered_apps";
pub const REGISTERED_APPS_KEYS: &str = "team_id, app_id, app_name, whitelisted_domains, ack_public_keys, email, registration_timestamp, pass_hash";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredApp {
    pub team_id: String,
    pub app_id: String,
    pub app_name: String,
    pub whitelisted_domains: Vec<String>,
    pub subscription: Option<Subscription>,
    pub ack_public_keys: Vec<String>,
    pub email: Option<String>,
    pub registration_timestamp: u64,
    pub pass_hash: Option<String>,
}

impl FromRow<'_, PgRow> for RegisteredApp {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        let registration_timestamp: i64 = row.get("registration_timestamp");
        Ok(RegisteredApp {
            team_id: row.get("team_id"),
            app_id: row.get("app_id"),
            app_name: row.get("app_name"),
            whitelisted_domains: row.get("whitelisted_domains"),
            // TEMP
            subscription: None,
            ack_public_keys: row.get("ack_public_keys"),
            email: row.get("email"),
            registration_timestamp: registration_timestamp as u64,
            pass_hash: row.get("pass_hash"),
        })
    }
}
