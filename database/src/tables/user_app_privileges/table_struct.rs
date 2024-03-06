use crate::structs::privilege_level::PrivilegeLevel;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const USER_APP_PRIVILEGES_TABLE_NAME: &str = "user_app_privileges";
pub const USER_APP_PRIVILEGES_KEYS: &str = "user_id, app_id, privilege_level, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAppPrivilege {
    pub user_id: String,
    pub app_id: String,
    pub privilege_level: PrivilegeLevel,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for UserAppPrivilege {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(UserAppPrivilege {
            user_id: row.get("user_id"),
            app_id: row.get("app_id"),
            privilege_level: row.get("privilege_level"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
