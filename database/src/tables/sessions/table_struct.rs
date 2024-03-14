use crate::structs::{client_data::ClientData, session_type::SessionType};
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const SESSIONS_TABLE_NAME: &str = "sessions";
pub const SESSIONS_KEYS: &str =
    "session_id, session_type, app_id, app_metadata, persistent, network, client_profile_id, client_id, client_device, client_metadata, client_notification_endpoint, client_connected_at, session_open_timestamp, session_close_timestamp";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbNcSession {
    pub session_id: String,
    pub session_type: SessionType,
    pub app_id: String,
    pub app_metadata: String,
    pub persistent: bool,
    pub network: String,
    pub client_profile_id: Option<String>,
    pub client: Option<ClientData>, // Some if user has ever connected to the session
    pub session_open_timestamp: DateTime<Utc>,
    pub session_close_timestamp: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for DbNcSession {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        let client_connected_at: Option<DateTime<Utc>> = row.get("client_connected_at");
        Ok(DbNcSession {
            app_id: row.get("app_id"),
            session_type: row.get("session_type"),
            app_metadata: row.get("app_metadata"),
            persistent: row.get("persistent"),
            network: row.get("network"),
            session_id: row.get("session_id"),
            client_profile_id: row.get("client_profile_id"),
            // If client has ever connected to the session, return the client data
            client: match client_connected_at {
                Some(client_connected_at) => Some(ClientData {
                    client_id: row.get("client_id"),
                    device: row.get("client_device"),
                    metadata: row.get("client_metadata"),
                    notification_endpoint: row.get("client_notification_endpoint"),
                    connected_at: client_connected_at,
                }),
                None => None,
            },
            session_open_timestamp: row.get("session_open_timestamp"),
            session_close_timestamp: row.get("session_close_timestamp"),
        })
    }
}
