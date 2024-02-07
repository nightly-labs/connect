use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UserConnectedEvent {
    pub public_keys: Vec<String>,
    #[ts(optional)]
    pub metadata: Option<String>,
}
