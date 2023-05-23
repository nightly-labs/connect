use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    initialize::{InitializeRequest, InitializeResponse},
    sign_transactions::SignTransactionsRequest,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppMessage {
    Initialize(InitializeRequest),
    SignTransactions(SignTransactionsRequest),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppResponse {
    Initialize(InitializeResponse),
}
