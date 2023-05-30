use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RequestRejected {
    #[serde(rename = "responseId")]
    pub response_id: String, // id of the request that was rejected
    #[ts(optional)]
    pub reason: Option<String>,
}
