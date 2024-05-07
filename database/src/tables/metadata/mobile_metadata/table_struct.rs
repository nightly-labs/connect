use sqlx::{postgres::PgRow, FromRow, Row};

pub const MOBILE_METADATA_TABLE_NAME: &str = "mobile_metadata";
pub const MOBILE_METADATA_KEYS: &str = "uuid, system_type, system_version";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbMobileMetadata {
    pub uuid: String,
    pub system_type: String,
    pub system_version: String,
}

impl FromRow<'_, PgRow> for DbMobileMetadata {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(DbMobileMetadata {
            uuid: row.get("uuid"),
            system_type: row.get("system_type"),
            system_version: row.get("system_version"),
        })
    }
}
