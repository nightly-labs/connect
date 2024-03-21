use crate::structs::request_status::RequestStatus;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_SIGN_AND_SEND_TRANSACTION_TABLE_NAME: &str = "event_sign_and_send_transaction";
pub const EVENT_SIGN_AND_SEND_TRANSACTION_KEYS: &str =
    "event_id, session_id, request_id, request_status, network, tx_hash";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignAndSendTransactionEvent {
    pub event_id: String,
    pub session_id: String,
    pub request_id: String,
    pub request_status: RequestStatus,
    pub network: String,
    pub tx_hash: Option<String>,
}

impl FromRow<'_, PgRow> for SignAndSendTransactionEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SignAndSendTransactionEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
            request_id: row.get("request_id"),
            request_status: row.get("request_status"),
            network: row.get("network"),
            tx_hash: row.get("tx_hash"),
        })
    }
}
