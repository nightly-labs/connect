use crate::structs::{
    entity_type::EntityType, geo_location::Geolocation, session_type::SessionType,
};
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const CONNECTION_EVENTS_TABLE_NAME: &str = "connection_events";
pub const CONNECTION_EVENTS_KEYS_KEYS: &str =
    "event_id, app_id, session_id, entity_id, entity_type, ip_address, session_type, geo_location, success, connected_at, disconnected_at";

#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionEvent {
    pub event_id: i64,
    pub app_id: String,
    pub session_id: String,
    pub entity_id: String,
    pub entity_type: EntityType,
    pub ip_address: String,
    pub session_type: Option<SessionType>,
    pub geo_location: Option<Geolocation>,
    pub success: bool,
    pub connected_at: DateTime<Utc>,
    pub disconnected_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for ConnectionEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(ConnectionEvent {
            event_id: row.get("event_id"),
            app_id: row.get("app_id"),
            session_id: row.get("session_id"),
            entity_id: row.get("entity_id"),
            entity_type: row.get("entity_type"),
            ip_address: row.get("ip_address"),
            session_type: row.get("session_type"),
            geo_location: row.get("geo_location"),
            success: row.get("success"),
            connected_at: row.get("connected_at"),
            disconnected_at: row.get("disconnected_at"),
        })
    }
}
