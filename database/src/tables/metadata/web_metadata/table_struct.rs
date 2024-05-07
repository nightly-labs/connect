use sqlx::{postgres::PgRow, FromRow, Row};

pub const WEB_METADATA_TABLE_NAME: &str = "web_metadata";
pub const WEB_METADATA_KEYS: &str = "uuid, browser, browser_version, os, os_version";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbWebMetadata {
    pub uuid: String,
    pub browser: String,
    pub browser_version: String,
    pub os: String,
    pub os_version: String,
}

impl FromRow<'_, PgRow> for DbWebMetadata {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(DbWebMetadata {
            uuid: row.get("uuid"),
            browser: row.get("browser"),
            browser_version: row.get("browser_version"),
            os: row.get("os"),
            os_version: row.get("os_version"),
        })
    }
}
