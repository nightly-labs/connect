use crate::structs::entity_type::EntityType;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const CONNECTION_EVENTS_TABLE_NAME: &str = "connection_events";
pub const CONNECTION_EVENTS_KEYS_KEYS: &str =
    "event_id, app_id, session_id, connection_id, entity_id, entity_type, ip_address, connected_at, disconnected_at";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionEvent {
    pub event_id: i64,
    pub app_id: String,
    pub session_id: String,
    pub connection_id: Option<String>,
    pub entity_id: String,
    pub entity_type: EntityType,
    pub ip_address: String,
    pub connected_at: DateTime<Utc>,
    pub disconnected_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for ConnectionEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ConnectionEvent {
            event_id: row.get("event_id"),
            app_id: row.get("app_id"),
            session_id: row.get("session_id"),
            connection_id: row.get("connection_id"),
            entity_id: row.get("entity_id"),
            entity_type: row.get("entity_type"),
            ip_address: row.get("ip_address"),
            connected_at: row.get("connected_at"),
            disconnected_at: row.get("disconnected_at"),
        })
    }
}
