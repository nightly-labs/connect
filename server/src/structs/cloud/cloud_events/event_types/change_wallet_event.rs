use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWalletEvent {
    pub session_id: String,
    pub request_id: String,
    pub network: String,
    pub wallet_name: String,
    pub wallet_type: String,
    pub old_wallet_address: String,
}
