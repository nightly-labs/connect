use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::{
    app_messages::sign_transactions::SignTransactionsRequest, common::SignedTransaction,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEvent {
    pub request: SignTransactionsRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SignTransactionsEventReply {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<SignedTransaction>,
    #[ts(optional)]
    pub metadata: Option<String>,
}
