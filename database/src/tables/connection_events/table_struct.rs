use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

use crate::structs::entity_type::EntityType;

pub const CONNECTION_EVENTS_TABLE_NAME: &str = "connection_events";
pub const CONNECTION_EVENTS_KEYS_KEYS: &str =
    "event_id, session_id, connection_id, entity_id, entity_type, network, connected_at, disconnected_at";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionEvent {
    pub event_id: i64,
    pub session_id: String,
    pub connection_id: Option<String>,
    pub entity_id: String,
    pub entity_type: EntityType,
    pub network: String,
    pub connected_at: DateTime<Utc>,
    pub disconnected_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for ConnectionEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ConnectionEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
            connection_id: row.get("connection_id"),
            entity_id: row.get("entity_id"),
            entity_type: row.get("entity_type"),
            network: row.get("network"),
            connected_at: row.get("connected_at"),
            disconnected_at: row.get("disconnected_at"),
        })
    }
}
