use crate::structs::request_status::RequestStatus;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_SIGN_MESSAGE_TABLE_NAME: &str = "event_sign_message";
pub const EVENT_SIGN_MESSAGE_KEYS: &str =
    "event_id, session_id, request_id, request_status, network";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignMessageEvent {
    pub event_id: String,
    pub session_id: String,
    pub request_id: String,
    pub request_status: RequestStatus,
    pub network: String,
}

impl FromRow<'_, PgRow> for SignMessageEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SignMessageEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
            request_id: row.get("request_id"),
            request_status: row.get("request_status"),
            network: row.get("network"),
        })
    }
}
