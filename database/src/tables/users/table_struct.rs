use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};
use webauthn_rs::prelude::Passkey;

pub const USERS_TABLE_NAME: &str = "users";
pub const USERS_KEYS: &str = "user_id, email, password_hash, passkeys, creation_timestamp";

#[derive(Clone, Debug, PartialEq)]
pub struct GrafanaUser {
    pub user_id: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub passkeys: Option<Vec<Passkey>>,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for GrafanaUser {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        let passkeys: Option<String> = row.get("passkeys");
        Ok(GrafanaUser {
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            passkeys: match passkeys {
                Some(passkeys) => {
                    serde_json::from_str(&passkeys).map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                }
                None => None,
            },
            user_id: row.get("user_id"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
