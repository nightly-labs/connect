use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const SESSION_PUBLIC_KEYS_TABLE_NAME: &str = "session_public_keys";
pub const SESSION_PUBLIC_KEYS_KEYS: &str =
    "session_public_key_id, session_id, public_key_id, timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionPublicKey {
    pub session_public_key_id: i64,
    pub session_id: String,
    pub public_key_id: i64,
    pub timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for SessionPublicKey {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SessionPublicKey {
            session_public_key_id: row.get("session_public_key_id"),
            session_id: row.get("session_id"),
            public_key_id: row.get("public_key_id"),
            timestamp: row.get("timestamp"),
        })
    }
}
