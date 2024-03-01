use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const PUBLIC_KEYS_TABLE_NAME: &str = "public_keys";
pub const PUBLIC_KEYS_KEYS: &str = "public_key, client_profile_id, first_seen, last_seen";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicKey {
    pub public_key: String,
    pub client_profile_id: i64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for PublicKey {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(PublicKey {
            public_key: row.get("public_key"),
            client_profile_id: row.get("client_profile_id"),
            first_seen: row.get("first_seen"),
            last_seen: row.get("last_seen"),
        })
    }
}
