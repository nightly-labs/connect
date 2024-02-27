use crate::structs::privelage_level::PrivilegeLevel;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const GRAFANA_USERS_TABLE_NAME: &str = "grafana_users";
pub const GRAFANA_USERS_KEYS: &str =
    "name, team_id, team_admin, email, password_hash, privilege_level, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrafanaUser {
    pub name: String,
    pub team_id: String,
    pub team_admin: bool,
    pub email: String,
    pub password_hash: String,
    pub privilege_level: PrivilegeLevel,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for GrafanaUser {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(GrafanaUser {
            team_id: row.get("team_id"),
            name: row.get("name"),
            team_admin: row.get("team_admin"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            privilege_level: row.get("privilege_level"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
