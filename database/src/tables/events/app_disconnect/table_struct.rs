use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENT_APP_DISCONNECT_TABLE_NAME: &str = "event_app_disconnect";
pub const EVENT_APP_DISCONNECT_KEYS: &str = "event_id, session_id";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppDisconnectEvent {
    pub event_id: String,
    pub session_id: String,
}

impl FromRow<'_, PgRow> for AppDisconnectEvent {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(AppDisconnectEvent {
            event_id: row.get("event_id"),
            session_id: row.get("session_id"),
        })
    }
}
