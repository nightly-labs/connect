use crate::structs::event_type::EventType;
use crate::structs::pagination_cursor::CursorParams;
use chrono::{DateTime, Utc};
use sqlx::types::chrono;
use sqlx::{postgres::PgRow, FromRow, Row};

pub const EVENTS_TABLE_NAME: &str = "events";
pub const EVENTS_KEYS: &str = "event_id, app_id, event_type, creation_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub event_id: i64,
    pub app_id: String,
    pub event_type: EventType,
    pub creation_timestamp: DateTime<Utc>,
}

impl CursorParams for Event {
    fn get_date(&self) -> DateTime<Utc> {
        self.creation_timestamp
    }

    fn get_id(&self) -> i64 {
        self.event_id
    }
}

impl FromRow<'_, PgRow> for Event {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Event {
            event_id: row.get("event_id"),
            app_id: row.get("app_id"),
            event_type: row.get("event_type"),
            creation_timestamp: row.get("creation_timestamp"),
        })
    }
}
