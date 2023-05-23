use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::Network;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsRequest {
    pub response_id: String,

    pub transactions: Vec<TransactionToSign>,
    pub network: Network,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransactionToSign {
    pub transaction: String, // serialized transaction
    #[serde(rename = "publicKey")]
    pub public_key: Vec<String>, // keys that need to sign the transaction
}
