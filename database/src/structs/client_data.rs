use sqlx::{
    types::chrono::{DateTime, Utc},
    Type,
};

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "client_data")]
pub struct ClientData {
    pub client_profile_id: i64,
    pub client_id: String,
    pub wallet_name: String,
    pub wallet_type: String,
    pub connected_at: DateTime<Utc>, // Timestamp of when the client connected to the session
}
