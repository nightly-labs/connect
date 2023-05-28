use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Network, TransactionToSign};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsRequest {
    pub response_id: String,

    pub transactions: Vec<TransactionToSign>,
    pub network: Network,
}
