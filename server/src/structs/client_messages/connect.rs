use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectRequest {
    pub response_id: String,
    #[serde(rename = "publicKey")]
    pub public_keys: Vec<String>,
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectResponse {
    pub response_id: String,
}
