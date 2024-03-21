use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_CLIENT_CONNECT_TABLE_NAME: &str = "event_client_connect";
pub const EVENT_CLIENT_CONNECT_KEYS: &str =
    "event_id, client_id, session_id, addresses, wallet_name, wallet_type, session_type, success";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientConnectEvent {
    pub event_id: String,
    pub client_id: String,
    pub session_id: String,
    pub addresses: Vec<String>,
    pub wallet_name: String,
    pub wallet_type: String,
    pub session_type: String,
    pub success: bool,
}

impl FromRow<'_, PgRow> for ClientConnectEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ClientConnectEvent {
            event_id: row.get("event_id"),
            client_id: row.get("client_id"),
            session_id: row.get("session_id"),
            addresses: row.get("addresses"),
            wallet_name: row.get("wallet_name"),
            wallet_type: row.get("wallet_type"),
            session_type: row.get("session_type"),
            success: row.get("success"),
        })
    }
}
