use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserConnectedEvent {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
    #[ts(optional)]
    pub metadata: Option<String>,
}
