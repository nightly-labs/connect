use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const TEAM_INVITES_TABLE_NAME: &str = "team_invites";
pub const TEAM_INVITES_KEYS: &str =
    "invite_id, team_id, user_email, created_at, accepted_at, cancelled_at";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamInvite {
    pub invite_id: i64,
    pub team_id: String,
    pub user_email: String,
    pub created_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    // Not present in the table, queried from the team table
    pub team_name: String,
    pub admin_email: String,
}

impl FromRow<'_, PgRow> for TeamInvite {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(TeamInvite {
            invite_id: row.get("invite_id"),
            team_id: row.get("team_id"),
            user_email: row.get("user_email"),
            created_at: row.get("created_at"),
            accepted_at: row.get("accepted_at"),
            cancelled_at: row.get("cancelled_at"),
            team_name: row.get("team_name"),
            admin_email: row.get("admin_email"),
        })
    }
}
