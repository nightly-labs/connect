use chrono::{DateTime, Utc};
use database::{structs::event_type::EventType, tables::events::events_index::table_struct::Event};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppEvent {
    pub event_type: EventType,
    pub creation_timestamp: DateTime<Utc>,
}

impl From<Event> for AppEvent {
    fn from(event: Event) -> Self {
        AppEvent {
            event_type: event.event_type,
            creation_timestamp: event.creation_timestamp,
        }
    }
}
