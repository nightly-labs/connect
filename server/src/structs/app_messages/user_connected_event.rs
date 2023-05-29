use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::Network;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserConnectedEvent {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
}
