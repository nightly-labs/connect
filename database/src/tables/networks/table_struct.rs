use sqlx::{postgres::PgRow, FromRow, Row};

pub const NETWORKS_TABLE_NAME: &str = "networks";
pub const NETWORKS_KEYS: &str = "network";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Network {
    pub network: String,
}

impl FromRow<'_, PgRow> for Network {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Network {
            network: row.get("network"),
        })
    }
}
