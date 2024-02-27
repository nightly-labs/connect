use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const GRAFANA_USERS_TABLE_NAME: &str = "grafana_users";
pub const GRAFANA_USERS_KEYS: &str = "user_id, email, password_hash, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrafanaUser {
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for GrafanaUser {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(GrafanaUser {
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            user_id: row.get("user_id"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
