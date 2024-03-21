use crate::structs::request_status::RequestStatus;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_CHANGE_WALLET_TABLE_NAME: &str = "event_change_wallet";
pub const EVENT_CHANGE_WALLET_KEYS: &str =
    "event_id, session_id, request_id, request_status, network, wallet_name, wallet_type, old_wallet_address, new_wallet_address";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeWalletEvent {
    pub event_id: i64,
    pub session_id: String,
    pub request_id: String,
    pub request_status: RequestStatus,
    pub network: String,
    pub wallet_name: String,
    pub wallet_type: String,
    pub old_wallet_address: String,
    pub new_wallet_address: Option<String>,
}

impl FromRow<'_, PgRow> for ChangeWalletEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ChangeWalletEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
            request_id: row.get("request_id"),
            request_status: row.get("request_status"),
            network: row.get("network"),
            wallet_name: row.get("wallet_name"),
            wallet_type: row.get("wallet_type"),
            old_wallet_address: row.get("old_wallet_address"),
            new_wallet_address: row.get("new_wallet_address"),
        })
    }
}
