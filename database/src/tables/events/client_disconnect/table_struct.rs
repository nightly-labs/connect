use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_CLIENT_DISCONNECT_TABLE_NAME: &str = "event_client_disconnect";
pub const EVENT_CLIENT_DISCONNECT_KEYS: &str = "event_id, client_id, disconnected_session_id";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientDisconnectEvent {
    pub event_id: i64,
    pub client_id: String,
    pub disconnected_session_id: String,
}

impl FromRow<'_, PgRow> for ClientDisconnectEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ClientDisconnectEvent {
            event_id: row.get("event_id"),
            client_id: row.get("client_id"),
            disconnected_session_id: row.get("disconnected_session_id"),
        })
    }
}
