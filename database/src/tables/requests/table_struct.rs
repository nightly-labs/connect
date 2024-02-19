use sqlx::{postgres::PgRow, FromRow, Row};

use crate::structs::request_status::RequestStatus;

pub const REQUESTS_TABLE_NAME: &str = "requests";
pub const REQUESTS_KEYS: &str =
    "request_id, request_type, session_id, request_status, network, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub request_id: String,
    pub request_type: String,
    pub session_id: String,
    pub request_status: RequestStatus,
    pub network: String,
    pub creation_timestamp: u64,
}

impl FromRow<'_, PgRow> for Request {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        let creation_timestamp: i64 = row.get("creation_timestamp");
        Ok(Request {
            request_id: row.get("request_id"),
            request_type: row.get("request_type"),
            session_id: row.get("session_id"),
            request_status: row.get("request_status"),
            network: row.get("network"),
            creation_timestamp: creation_timestamp as u64,
        })
    }
}
