use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Network(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Version(pub String); // 0.0.1

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SessionStatus {
    WaitingForClient,
    Connected,
    UserDisconnected,
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Device {
    Apple,
    Android,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransactionToSign {
    pub transaction: String, // serialized transaction
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>, // keys that need to sign the transaction
    pub network: Network,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignedTransaction {
    pub transaction: String, // serialized transaction
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>, // keys that signed the transaction
    pub network: Network,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ErrorMessage {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub error: String,
}
