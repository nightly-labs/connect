use crate::structs::request_status::RequestStatus;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_CHANGE_NETWORK_TABLE_NAME: &str = "event_change_network";
pub const EVENT_CHANGE_NETWORK_KEYS: &str =
    "event_id, session_id, request_id, request_status, old_network, new_network";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeNetworkEvent {
    pub event_id: i64,
    pub session_id: String,
    pub request_id: String,
    pub request_status: RequestStatus,
    pub old_network: String,
    pub new_network: Option<String>,
}

impl FromRow<'_, PgRow> for ChangeNetworkEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ChangeNetworkEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
            request_id: row.get("request_id"),
            request_status: row.get("request_status"),
            old_network: row.get("old_network"),
            new_network: row.get("new_network"),
        })
    }
}
