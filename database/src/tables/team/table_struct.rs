use crate::structs::subscription::Subscription;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const TEAM_TABLE_NAME: &str = "team";
pub const TEAM_KEYS: &str =
    "team_id, personal, subscription, team_admin_id, registration_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Team {
    pub team_id: String,
    pub personal: bool,
    // Subscription is required to get access to the statistics
    pub subscription: Option<Subscription>,
    pub team_admin_id: String,
    pub registration_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for Team {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Team {
            team_id: row.get("team_id"),
            personal: row.get("personal"),
            subscription: row.get("subscription"),
            registration_timestamp: row.get("registration_timestamp"),
            team_admin_id: row.get("team_admin_id"),
        })
    }
}
