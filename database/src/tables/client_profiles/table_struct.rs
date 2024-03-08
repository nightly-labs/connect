use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const CLIENT_PROFILES_TABLE_NAME: &str = "client_profiles";
pub const CLIENT_PROFILES_KEYS: &str = "client_profile_id, created_at";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientProfile {
    pub client_profile_id: i64,
    pub created_at: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for ClientProfile {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ClientProfile {
            client_profile_id: row.get("client_profile_id"),
            created_at: row.get("created_at"),
        })
    }
}
