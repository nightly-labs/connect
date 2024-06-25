use crate::structs::medium_type::DeviceMediumType;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_APP_CONNECT_TABLE_NAME: &str = "event_app_connect";
pub const EVENT_APP_CONNECT_KEYS: &str =
    "event_id, app_id, network, session_id, device_medium_type, device_metadata_uuid, lang, timezone, new_session, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppConnectEvent {
    pub event_id: i64,
    pub network: String,
    pub session_id: String,
    pub device_medium_type: DeviceMediumType,
    pub device_metadata_uuid: String,
    pub lang: String,
    pub timezone: String,
}

impl FromRow<'_, PgRow> for AppConnectEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(AppConnectEvent {
            event_id: row.get("event_id"),
            network: row.get("network"),
            session_id: row.get("session_id"),
            device_medium_type: row.get("device_medium_type"),
            device_metadata_uuid: row.get("device_metadata_uuid"),
            lang: row.get("lang"),
            timezone: row.get("timezone"),
        })
    }
}
