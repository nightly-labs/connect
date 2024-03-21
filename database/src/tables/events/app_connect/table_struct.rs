use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_APP_CONNECT_TABLE_NAME: &str = "event_app_connect";
pub const EVENT_APP_CONNECT_KEYS: &str =
    "event_id, session_id, device_metadata, lang, timezone, new_session";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppConnectEvent {
    pub event_id: i64,
    pub app_id: String,
    pub session_id: String,
    pub device_metadata: String,
    pub lang: String,
    pub timezone: String,
}

impl FromRow<'_, PgRow> for AppConnectEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(AppConnectEvent {
            event_id: row.get("event_id"),
            app_id: row.get("app_id"),
            session_id: row.get("session_id"),
            device_metadata: row.get("device_metadata"),
            lang: row.get("lang"),
            timezone: row.get("timezone"),
        })
    }
}
