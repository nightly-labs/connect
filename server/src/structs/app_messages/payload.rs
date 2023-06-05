use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RequestPayload {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub content: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResponsePayload {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub content: String,
}
