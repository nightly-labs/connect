use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ClientInitializeRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ClientInitializeResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
}
