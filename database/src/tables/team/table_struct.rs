use crate::structs::subscription::Subscription;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const TEAM_TABLE_NAME: &str = "team";
pub const TEAM_KEYS: &str = "team_id, subscription, team_admin_id, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Team {
    pub team_id: String,
    // Subscription is required to get access to the statistics
    pub subscription: Option<Subscription>,
    pub team_admin_id: String,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for Team {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Team {
            team_id: row.get("team_id"),
            subscription: row.get("subscription"),
            creation_timestamp: row.get("creation_timestamp"),
            team_admin_id: row.get("team_admin_id"),
        })
    }
}
