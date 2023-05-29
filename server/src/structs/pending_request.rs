use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::app_messages::{
    sign_messages::SignMessagesRequest, sign_transactions::SignTransactionsRequest,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum PendingRequest {
    SignTransactions(SignTransactionsRequest),
    SignMessages(SignMessagesRequest),
}
