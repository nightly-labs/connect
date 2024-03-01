use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const SESSION_PUBLIC_KEYS_TABLE_NAME: &str = "session_public_keys";
pub const SESSION_PUBLIC_KEYS_KEYS: &str =
    "session_public_key_id, session_id, public_key, key_belongs_to_profile, main_session_key, timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionPublicKey {
    pub session_public_key_id: i64,
    pub session_id: String,
    pub public_key: String,
    pub key_belongs_to_profile: i64,
    pub main_session_key: bool,
    pub timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for SessionPublicKey {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SessionPublicKey {
            session_public_key_id: row.get("session_public_key_id"),
            session_id: row.get("session_id"),
            public_key: row.get("public_key"),
            key_belongs_to_profile: row.get("key_belongs_to_profile"),
            main_session_key: row.get("main_session_key"),
            timestamp: row.get("timestamp"),
        })
    }
}
