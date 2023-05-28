use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{SignedTransaction, TransactionToSign};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEvent {
    pub transactions: Vec<TransactionToSign>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEventReply {
    pub signed_transactions: Vec<SignedTransaction>,
}
