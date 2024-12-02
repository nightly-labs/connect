use std::str::FromStr;

use crate::structs::request_status::RequestStatus;
use crate::structs::request_type::RequestType;
use chrono::{DateTime, Utc};
use sqlx::types::chrono;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const REQUESTS_TABLE_NAME: &str = "requests";
pub const REQUESTS_KEYS: &str =
    "request_id, session_id, app_id, request_type, request_status, network, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub request_id: String,
    pub session_id: String,
    pub app_id: String,
    pub request_type: RequestType,
    pub request_status: RequestStatus,
    pub network: String,
    pub creation_timestamp: DateTime<Utc>,
}

impl FromRow<'_, PgRow> for Request {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Request {
            request_id: row.get("request_id"),
            app_id: row.get("app_id"),
            request_type: RequestType::from_str(row.get("request_type"))
                .map_err(|_| sqlx::Error::Decode(format!("Invalid request_type")))?,
            session_id: row.get("session_id"),
            request_status: RequestStatus::from_str(row.get("request_status"))
                .map_err(|_| sqlx::Error::Decode(format!("Invalid request_status")))?,
            network: row.get("network"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
