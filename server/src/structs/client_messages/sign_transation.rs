use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{SignedTransaction, TransactionToSign};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEvent {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub transactions: Vec<TransactionToSign>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEventReply {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<SignedTransaction>,
}
