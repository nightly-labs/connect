use database::structs::session_type::SessionType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnectInitEvent {
    pub client_id: String,
    pub session_id: String,
    pub wallet_name: String,
    pub wallet_type: String,
    pub session_type: SessionType,
}
